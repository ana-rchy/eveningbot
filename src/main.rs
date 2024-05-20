use eveningbot::{global::*, jobs, commands, event, web};
use poise::serenity_prelude::{self as serenity, GatewayIntents};
use std::sync::{Arc, Mutex};
use tokio_cron_scheduler::JobScheduler;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_data = SharedData {
        sunset_time: Arc::new(Mutex::new(web::get_sunset_time().await.unwrap()))
    };

    let mut client = poise_setup(&shared_data).await;
    let sched: Arc<JobScheduler> = Arc::new(JobScheduler::new().await?);

    jobs::init_jobs(sched.clone(), &client, &shared_data).await?;

    sched.start().await?;
    client.start().await.unwrap();

    Ok(())
}

pub async fn poise_setup(shared_data: &SharedData) -> serenity::Client {
    let token = std::env::var("DISCORD_TOKEN").expect("envvar the DISCORD_TOKEN");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES;

    let sunset_time = shared_data.sunset_time.clone();

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(SharedData {
                    sunset_time
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![commands::fact_check()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            event_handler: move |ctx, event, framework, data| {
                Box::pin(event::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .build();

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap()
}
