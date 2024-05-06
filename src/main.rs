use poise::serenity_prelude::{self as serenity, CreateMessage};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use eveningbot::motd::*;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    let mut client = poise_setup().await;
    let sched = JobScheduler::new().await?;

    add_jobs(&sched, &client).await?;

    sched.start().await?;
    client.start().await.unwrap();

    Ok(())
}


async fn poise_setup() -> serenity::Client {
    let token = std::env::var("DISCORD_TOKEN").expect("envvar the DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    
    let framework = poise::Framework::<(), Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .build();

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap()
}

async fn add_jobs(sched: &JobScheduler, client: &serenity::Client) -> Result<(), JobSchedulerError> {
    const GENERAL_CHANNEL_ID: u64 = 1215048710074011692;
    const TESTING_CHANNEL_ID: u64 = 1235087573421133824;
    let http = client.http.clone();
    
    // night - 3am
    {
        let channel = serenity::ChannelId::new(GENERAL_CHANNEL_ID);
        let mut bag = NIGHT_MOTD.to_vec();

        sched.add(
            Job::new_async("0 0 3 * * *", move |_uuid, _l| {
                let http = http.clone();

                if bag.is_empty() {
                    bag = NIGHT_MOTD.to_vec();
                }

                let rand_index = rand::random::<usize>() % bag.len();
                let motd = bag[rand_index];
                bag.remove(rand_index);

                let message = CreateMessage::new()
                    .content(motd);

                Box::pin(async move {
                    let _ = channel.send_message(&http, message).await;
                })
            })?
        ).await?;
    }

    Ok(())
}
