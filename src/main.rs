use eveningbot::{commands, jobs};
use poise::serenity_prelude::{self as serenity, CreateMessage, GuildRef, GatewayIntents};
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;

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
    let intents =  GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::<(), Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::fact_check()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, _| {
                Box::pin(event_handler(ctx, event, framework))
            },
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

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, (), Error>
) -> Result<(), Error> {
    const USERS_REMOVED_CHANNEL_ID: u64 = 1240091460217344100;
    let channel = serenity::ChannelId::new(USERS_REMOVED_CHANNEL_ID);

    match event {
        serenity::FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            let member_count = {
                let guild: GuildRef = ctx.cache.guild(guild_id).unwrap();
                guild.member_count
            };

            let message = CreateMessage::new()
                .content(
                    format!("<@{}> left, now {} server members",
                        user.id,
                        member_count)
                    );

            let _ = channel.send_message(&ctx.http, message).await;
        }

        _ => {}
    }

    Ok(())
}
