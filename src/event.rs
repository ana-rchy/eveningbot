use crate::global::*;
use log::{debug, info};
use poise::serenity_prelude::{self as serenity, CreateMessage, EmojiId, GuildRef, ReactionType};
use std::str::FromStr;
use std::sync::atomic::Ordering;
use time::*;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, SharedData, Error>,
    shared_data: &SharedData,
) -> std::prelude::v1::Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            const USERS_REMOVED_CHANNEL_ID: u64 = 1240091460217344100;
            let channel = serenity::ChannelId::new(USERS_REMOVED_CHANNEL_ID);

            let member_count = {
                let guild: GuildRef = ctx.cache.guild(guild_id).unwrap();
                guild.member_count
            };

            let content = format!("<@{}> left, now {} server members", user.id, member_count);

            let message = CreateMessage::new().content(&content);
            let _ = channel.send_message(&ctx.http, message).await;

            info!("{content}");
        }

        serenity::FullEvent::Message { new_message } => {
            easter_egg_reacts(&ctx, &new_message).await;

            // early returns
            #[allow(dead_code)]
            const TESTING_CHANNEL_ID: u64 = 1235087573421133824;
            const GENERAL_CHANNEL_ID: u64 = 1215048710074011692;
            const BOT_ID: u64 = 1235086289255137404;

            let sunset_time = *shared_data.sunset_time.lock().unwrap();
            let current_time = OffsetDateTime::now_utc().to_offset(sunset_time.offset());

            if !(current_time.time() > sunset_time.time() && current_time.hour() < 24)
                || !(new_message.channel_id == GENERAL_CHANNEL_ID
                    || new_message.channel_id == TESTING_CHANNEL_ID)
                || new_message.author.id == BOT_ID
                || !GOOD_EVENINGS
                    .iter()
                    .any(|a| new_message.content.to_lowercase().contains(a))
            {
                return Ok(());
            }

            // react to good evenings
            let reaction = ReactionType::Custom {
                animated: false,
                id: EmojiId::new(1241916769648775238),
                name: Some("eepy".to_string()),
            };

            new_message.react(&ctx.http, reaction).await.unwrap();

            debug!("GE reaction added for message: {}", new_message.content);

            // handle leaderboard if its the first GE of the day
            if shared_data.first_ge_sent.load(Ordering::SeqCst) {
                return Ok(());
            }

            shared_data.first_ge_sent.store(true, Ordering::SeqCst);

            let user_id = u64::from(new_message.author.id);
            let mut leaderboard = shared_data.evening_leaderboard.lock().await;

            let path = format!("{}/assets/leaderboard.bin", shared_data.root_path);
            if leaderboard.is_empty() {
                debug!("leaderboard hashmap is empty");
            } else {
                debug!("leaderboard hashmap is filled");
            }
            if std::path::Path::new(&path).exists() {
                debug!("leaderboard file exists");
            } else {
                debug!("leaderboard file doesnt exist");
            }

            leaderboard
                .entry(user_id)
                .and_modify(|e| *e += 1)
                .or_insert(1);

            let leaderboard_bytes =
                rmp_serde::encode::to_vec(&*leaderboard).expect("couldnt serialize leaderboard");
            _ = std::fs::write(path, leaderboard_bytes);

            info!("first GE of day sent, leaderboard written to");
        }

        _ => {}
    }

    Ok(())
}

async fn easter_egg_reacts(ctx: &serenity::Context, message: &serenity::model::channel::Message) {
    for i in EASTER_EGG_REACTS.entries() {
        let msg = &message.content.to_lowercase();

        if !msg.contains(i.0) {
            continue;
        }

        // dont react if word surrounded by alphabetical characters
        let egg_index = msg.find(i.0).unwrap();
        if (egg_index != 0 && msg.chars().nth(egg_index - 1).unwrap().is_alphabetic())
            || (egg_index + i.0.len() != msg.len()
                && msg
                    .chars()
                    .nth(egg_index + i.0.len())
                    .unwrap()
                    .is_alphabetic())
        {
            continue;
        }

        let reaction = ReactionType::from_str(i.1).unwrap();

        message.react(&ctx.http, reaction).await.unwrap();

        debug!("easter egg reaction {} added", i.1);
    }
}
