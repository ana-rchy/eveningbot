use std::sync::Arc;
use poise::serenity_prelude::{self as serenity};
use tokio_cron_scheduler::JobScheduler;
use eveningbot::jobs;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = poise_setup().await;
    let sched: Arc<JobScheduler> = Arc::new(JobScheduler::new().await?);

    jobs::init_jobs(sched.clone(), &client).await?;

    sched.start().await?;
    client.start().await.unwrap();

    Ok(())
}



pub async fn poise_setup() -> serenity::Client {
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
