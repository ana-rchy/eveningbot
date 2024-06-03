use eveningbot::{global::*, jobs, commands::*, event};
use poise::serenity_prelude::{self as serenity, GatewayIntents};
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_allow_str("eveningbot")
        .build();
    simplelog::WriteLogger::init(
        log::LevelFilter::Debug,
        log_config, 
        std::fs::File::create("eveningbot.log").unwrap())?;

    let shared_data = SharedData::new().await;

    let mut client = poise_setup(&shared_data).await;
    let sched: Arc<JobScheduler> = Arc::new(JobScheduler::new().await?);

    jobs::init_jobs(sched.clone(), &client, &shared_data).await?;

    sched.start().await.expect("scheduler failed");
    client.start().await.expect("client failed");

    Ok(())
}

pub async fn poise_setup(shared_data: &SharedData) -> serenity::Client {
    let token = std::env::var("DISCORD_TOKEN").expect("envvar the DISCORD_TOKEN");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES;

    let sunset_time = shared_data.sunset_time.clone();
    let assets_path = shared_data.root_path.clone();
    let evening_leaderboard = shared_data.evening_leaderboard.clone();
    let first_ge_sent = shared_data.first_ge_sent.clone();

    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(SharedData {
                    sunset_time,
                    root_path: assets_path,
                    evening_leaderboard,
                    first_ge_sent
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![fact_check(), get_leaderboard(), roll(), uwuify()],
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
