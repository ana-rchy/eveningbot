use crate::global::*;
use crate::web;
use log::debug;
use poise::serenity_prelude::{self as serenity, Colour, CreateEmbed, CreateMessage, Http, UserId};
use std::sync::{atomic::Ordering, Arc, Mutex};
use tokio_cron_scheduler::{JobBuilder, JobScheduler, JobSchedulerError};
use uuid::Uuid;

pub async fn init_jobs(
    sched: Arc<JobScheduler>,
    client: &serenity::Client,
    shared_data: &SharedData,
) -> Result<(), JobSchedulerError> {
    #[allow(dead_code)]
    const TESTING_CHANNEL_ID: u64 = 1235087573421133824;
    const GENERAL_CHANNEL_ID: u64 = 1215048710074011692;

    let http = client.http.clone();

    // arc my beloved
    let sunset_job_id: Arc<Mutex<Uuid>> = Default::default();
    let evening_bag = Arc::new(Mutex::new(EVENING_MOTD.to_vec()));
    let night_bag = Arc::new(Mutex::new(NIGHT_MOTD.to_vec()));

    // evening - sunset time from sunrise-sunset.org
    {
        // braces to ensure lock gets dropped
        let sunset_time = { *shared_data.sunset_time.lock().unwrap() };

        let job = create_sunset_job(
            http.clone(),
            GENERAL_CHANNEL_ID,
            sunset_time,
            evening_bag.clone(),
        )
        .await;
        {
            *sunset_job_id.lock().unwrap() = sched.add(job).await?;
        }
    }

    // refresh sunset job/time - 12:02am
    {
        let http = http.clone();
        let sched_closure = sched.clone();
        let sunset_time = shared_data.sunset_time.clone();

        let job = JobBuilder::new()
            .with_timezone(chrono_tz::Europe::Dublin)
            .with_cron_job_type()
            .with_schedule("0 2 0 * * *") // leeway for api to update
            .unwrap()
            .with_run_async(Box::new(move |_uuid, _l| {
                let http = http.clone();
                let sched = sched_closure.clone();

                let sunset_time = sunset_time.clone();
                let sunset_job_id = sunset_job_id.clone();

                let evening_bag = evening_bag.clone();

                Box::pin(async move {
                    let new_sunset_time = web::get_sunset_time().await.unwrap();

                    let id = { *sunset_job_id.lock().unwrap() };
                    let job = create_sunset_job(
                        http,
                        GENERAL_CHANNEL_ID,
                        new_sunset_time,
                        evening_bag.clone(),
                    )
                    .await;

                    _ = sched.remove(&id).await;
                    let new_id = sched.add(job).await.unwrap();

                    {
                        *sunset_time.lock().unwrap() = new_sunset_time;
                    }
                    {
                        *sunset_job_id.lock().unwrap() = new_id;
                    }

                    debug!("sunset job {id} replaced with {new_id}");
                })
            }))
            .build()
            .unwrap();

        sched.clone().add(job).await?;
    }

    // night - 3am
    {
        let http = http.clone();
        let channel = serenity::ChannelId::new(GENERAL_CHANNEL_ID);

        let job = JobBuilder::new()
            .with_timezone(chrono_tz::Europe::Dublin)
            .with_cron_job_type()
            .with_schedule("0 0 3 * * *")
            .unwrap()
            .with_run_async(Box::new(move |_uuid, _l| {
                let http = http.clone();
                let mut bag = { night_bag.lock().unwrap() };

                if bag.is_empty() {
                    *bag = NIGHT_MOTD.to_vec();
                }

                let rand_index = rand::random::<usize>() % bag.len();
                let motd = bag[rand_index];
                bag.remove(rand_index);

                let message = CreateMessage::new().content("Nothing ever happens.");

                Box::pin(async move {
                    _ = channel.send_message(http, message).await;
                })
            }))
            .build()
            .unwrap();

        sched.add(job).await?;
    }

    Ok(())
}

async fn create_sunset_job(
    http: Arc<Http>,
    channel_id: u64,
    time: time::OffsetDateTime,
    bag: Arc<Mutex<Vec<&'static str>>>,
) -> tokio_cron_scheduler::Job {
    let schedule = &format!("{} {} {} * * *", time.second(), time.minute(), time.hour())[..];
    let channel = serenity::ChannelId::new(channel_id);

    debug!("creating new sunset job with schedule {schedule}");

    JobBuilder::new()
        .with_timezone(chrono_tz::Europe::Dublin)
        .with_cron_job_type()
        .with_schedule(schedule)
        .unwrap()
        .with_run_async(Box::new(move |_uuid, _l| {
            let http = http.clone();
            let mut bag = { bag.lock().unwrap() };

            if bag.is_empty() {
                *bag = EVENING_MOTD.to_vec();
            }

            let rand_index = rand::random::<usize>() % bag.len();
            let motd = bag[rand_index];
            bag.remove(rand_index);

            let ge_message = CreateMessage::new().content("Nothing ever happens.");

            Box::pin(async move {
                _ = channel.send_message(http, ge_message).await;
            })
        }))
        .build()
        .unwrap()
}
