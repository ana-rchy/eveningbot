use crate::global::*;
use poise::serenity_prelude::{self as serenity, CreateMessage, GuildRef, ReactionType, EmojiId};
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
            const GENERAL_CHANNEL_ID: u64 = 1215048710074011692;
            
            let sunset_time = *shared_data.sunset_time.lock().unwrap();
            let current_time = OffsetDateTime::now_utc().to_offset(sunset_time.offset());

            if !((current_time > sunset_time && current_time.hour() < 24) || current_time.hour() < 3)
                || new_message.channel_id != GENERAL_CHANNEL_ID
            {
                return Ok(());
            }

            let reaction = ReactionType::Custom {
                animated: false,
                id: EmojiId::new(1241916769648775238),
                name: Some("eepy".to_string()),
            };
            
            if GOOD_EVENINGS.contains(&&new_message.content.to_lowercase()[..]) {
                new_message.react(&ctx.http, reaction).await.unwrap();
            }
        }

        _ => {}
    }

    Ok(())
}
