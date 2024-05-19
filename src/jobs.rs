use crate::motd::*;
use crate::web;
use poise::serenity_prelude::Http;
use poise::serenity_prelude::{self as serenity, CreateMessage};
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::{JobBuilder, JobScheduler, JobSchedulerError};
use uuid::Uuid;

pub async fn init_jobs(
    sched: Arc<JobScheduler>,
    client: &serenity::Client,
) -> Result<(), JobSchedulerError> {
    const GENERAL_CHANNEL_ID: u64 = 1215048710074011692;
    #[allow(dead_code)]
    const TESTING_CHANNEL_ID: u64 = 1235087573421133824;
    let http = client.http.clone();
    
    // arc my beloved
    let sunset_job_id: Arc<Mutex<Uuid>> = Default::default();
    let evening_bag = Arc::new(Mutex::new(EVENING_MOTD.to_vec()));
    let night_bag = Arc::new(Mutex::new(NIGHT_MOTD.to_vec()));

    // evening - sunset time from sunrise-sunset.org
    {
        let job = create_sunset_job(http.clone(), GENERAL_CHANNEL_ID, evening_bag.clone()).await;
        *sunset_job_id.lock().unwrap() = sched.add(job).await?;
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
                let mut bag = night_bag.lock().unwrap();

                if bag.is_empty() {
                    *bag = NIGHT_MOTD.to_vec();
                }

                let rand_index = rand::random::<usize>() % bag.len();
                let motd = bag[rand_index];
                bag.remove(rand_index);

                let message = CreateMessage::new().content(motd);

                Box::pin(async move {
                    let _ = channel.send_message(http, message).await;
                })
            }))
            .build()
            .unwrap();

        sched.add(job).await?;
    }

    // refresh sunset job
    {
        let http = http.clone();
        let scope_sched = sched.clone();

        let job = JobBuilder::new()
            .with_timezone(chrono_tz::Europe::Dublin)
            .with_cron_job_type()
            .with_schedule("0 2 0 * * *") // leeway for api to update
            .unwrap()
            .with_run_async(Box::new(move |_uuid, _l| {
                let http = http.clone();
                let sched = sched.clone();
                let sunset_job_id = sunset_job_id.clone();

                let evening_bag = evening_bag.clone();

                Box::pin(async move {
                    // drop lock before sunset_job_id is used again
                    // otherwise youre tryna use it while its locked
                    let id = { *sunset_job_id.lock().unwrap() };

                    let _ = sched.remove(&id).await;

                    let job =
                        create_sunset_job(http, GENERAL_CHANNEL_ID, evening_bag.clone()).await;
                    let new_id = sched.add(job).await.unwrap();
                    // can be done without braces but better for consistency
                    { *sunset_job_id.lock().unwrap() = new_id; }
                })
            }))
            .build()
            .unwrap();

        scope_sched.add(job).await?;
    }

    Ok(())
}

async fn create_sunset_job(
    http: Arc<Http>,
    channel_id: u64,
    bag: Arc<Mutex<Vec<&'static str>>>,
) -> tokio_cron_scheduler::Job {
    let time = web::get_sunset_time().await.unwrap();
    let schedule = &format!("{} {} {} * * *", time.second(), time.minute(), time.hour())[..];
    println!(
        "sunset today - {:02}:{:02}:{:02}",
        time.hour(),
        time.minute(),
        time.second()
    );

    let channel = serenity::ChannelId::new(channel_id);

    JobBuilder::new()
        .with_timezone(chrono_tz::Europe::Dublin)
        .with_cron_job_type()
        .with_schedule(schedule)
        .unwrap()
        .with_run_async(Box::new(move |_uuid, _l| {
            let http = http.clone();
            let mut bag = bag.lock().unwrap();

            if bag.is_empty() {
                *bag = EVENING_MOTD.to_vec();
            }

            let rand_index = rand::random::<usize>() % bag.len();
            let motd = bag[rand_index];
            bag.remove(rand_index);

            let main_message = CreateMessage::new().content(motd);
            let ge_message = CreateMessage::new().content("good evening");

            Box::pin(async move {
                let _ = channel.send_message(&http, main_message).await;
                let _ = channel.send_message(http, ge_message).await;
            })
        }))
        .build()
        .unwrap()
}
