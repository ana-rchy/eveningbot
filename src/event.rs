use crate::global::*;
use std::sync::atomic::Ordering;
use poise::serenity_prelude::{self as serenity, CreateMessage, EmojiId, GuildRef, ReactionType};
use time::*;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, SharedData, Error>,
    shared_data: &SharedData
) -> std::prelude::v1::Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberRemoval { guild_id, user, .. } => {
            const USERS_REMOVED_CHANNEL_ID: u64 = 1240091460217344100;
            let channel = serenity::ChannelId::new(USERS_REMOVED_CHANNEL_ID);

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

        serenity::FullEvent::Message { new_message } => {
            // early returns
            const BOT_ID: u64 = 1235086289255137404;
            #[allow(dead_code)]
            const TESTING_CHANNEL_ID: u64 = 1235087573421133824;
            const GENERAL_CHANNEL_ID: u64 = 1215048710074011692;

            let sunset_time = *shared_data.sunset_time.lock().unwrap();
            let current_time = OffsetDateTime::now_utc().to_offset(sunset_time.offset());

            if !(current_time > sunset_time && current_time.hour() < 24)
                || !(new_message.channel_id == GENERAL_CHANNEL_ID || new_message.channel_id == TESTING_CHANNEL_ID)
                || new_message.author.id == BOT_ID
                || !GOOD_EVENINGS.iter().any(|a| new_message.content.to_lowercase().contains(a))
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


            // handle leaderboard if its the first GE of the day
            if shared_data.first_ge_sent.load(Ordering::SeqCst) {
                return Ok(());
            }

            shared_data.first_ge_sent.store(true, Ordering::SeqCst);

            let user_id = u64::from(new_message.author.id);
            let mut leaderboard = shared_data.evening_leaderboard.lock().await;

            leaderboard.entry(user_id).and_modify(|e| *e += 1).or_insert(1);

            let leaderboard_bytes = rmp_serde::encode::to_vec(&*leaderboard).expect("couldnt serialize leaderboard");
            _ = std::fs::write(format!("{}/assets/leaderboard.bin", shared_data.root_path), leaderboard_bytes);
        }

        _ => {}
    }

    Ok(())
}
