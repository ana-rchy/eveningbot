use crate::global::*;
use crate::web;
use std::sync::{Arc, Mutex, atomic::Ordering};
use poise::serenity_prelude::{self as serenity, Colour, CreateEmbed, CreateMessage, Http, UserId};
use tokio_cron_scheduler::{JobBuilder, JobScheduler, JobSchedulerError};
use uuid::Uuid;

pub async fn init_jobs(
    sched: Arc<JobScheduler>,
    client: &serenity::Client,
    shared_data: &SharedData
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

        let job = create_sunset_job(http.clone(), GENERAL_CHANNEL_ID, sunset_time, evening_bag.clone()).await;
        { *sunset_job_id.lock().unwrap() = sched.add(job).await?; }
    }

    // show leaderboard - 12am
    {
        let http = http.clone();
        let channel = serenity::ChannelId::new(GENERAL_CHANNEL_ID);
        let leaderboard = shared_data.evening_leaderboard.clone();
        let first_ge_sent = shared_data.first_ge_sent.clone();

        let job = JobBuilder::new()
            .with_timezone(chrono_tz::Europe::Dublin)
            .with_cron_job_type()
            .with_schedule("0 0 0 * * *")
            .unwrap()
            .with_run_async(Box::new(move |_uuid, _l| {
                let http = http.clone();
                let leaderboard = leaderboard.clone();
                
                // reset first ge message lock
                first_ge_sent.store(false, Ordering::SeqCst);

                Box::pin(async move {
                    let leaderboard = { leaderboard.lock().await };
                        
                    let leaderboard_top_10: String = 'top_10: {
                        if leaderboard.len() == 0 {
                            break 'top_10 "noone yet :(".to_string();
                        }

                        let mut sorted: Vec<(&u64, &u16)> = leaderboard.iter().collect();
                        sorted.sort_by(|a, b| b.1.cmp(a.1));
                        sorted.shrink_to(10);

                        let mut top_10 = String::new();
                        let mut position = 1;
                        for (id, count) in sorted {
                            let user_id = UserId::new(id.clone());
                            let user = user_id.to_user(http.clone()).await.expect("couldnt get user from id for leaderboard");
                            let username = user.global_name.unwrap();

                            top_10.push_str(&format!("{}. {}: {}\n", position, username, count));

                            position += 1;
                        }
                        top_10.remove(top_10.len() - 1);

                        top_10
                    };

                    let embed = CreateEmbed::new()
                        .colour(Colour::from_rgb(255, 0, 124))
                        .title("good evening leaderboard")
                        .description(leaderboard_top_10);
                    let message = CreateMessage::new()
                        .add_embed(embed);

                    _ = channel.send_message(http, message).await;
                })
            }))
            .build()
            .unwrap();

        sched.clone().add(job).await?;
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
                    let job = create_sunset_job(http, GENERAL_CHANNEL_ID, new_sunset_time, evening_bag.clone()).await;

                    _ = sched.remove(&id).await;
                    let new_id = sched.add(job).await.unwrap();

                    { *sunset_time.lock().unwrap() = new_sunset_time; }
                    { *sunset_job_id.lock().unwrap() = new_id; }
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

                let message = CreateMessage::new().content(motd);

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
    bag: Arc<Mutex<Vec<&'static str>>>
) -> tokio_cron_scheduler::Job {
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
            let mut bag = { bag.lock().unwrap() };

            if bag.is_empty() {
                *bag = EVENING_MOTD.to_vec();
            }

            let rand_index = rand::random::<usize>() % bag.len();
            let motd = bag[rand_index];
            bag.remove(rand_index);

            let main_message = CreateMessage::new().content(motd);
            let ge_message = CreateMessage::new().content("good evening");

            Box::pin(async move {
                _ = channel.send_message(&http, main_message).await;
                _ = channel.send_message(http, ge_message).await;
            })
        }))
        .build()
        .unwrap()
}
