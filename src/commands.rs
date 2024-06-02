use crate::global::*;
use poise::serenity_prelude::{ChannelId, Colour, CreateAttachment, CreateEmbed, CreateMessage, UserId};
use poise::reply::CreateReply;
use tokio::fs::File;
use log::debug;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, SharedData, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn fact_check(ctx: Context<'_>) -> Result<(), Error> {
    let (image, filename) = get_fact_check_image(ctx.data()).await;
    let attachment = CreateAttachment
        ::file(&image, filename).await.unwrap();

    let reply = CreateReply {
        attachments: vec![attachment],
        ..Default::default()
    };

    ctx.send(reply).await?;
    
    Ok(())
}

async fn get_fact_check_image(shared_data: &SharedData) -> (File, String) {
    let root_folder = &shared_data.root_path;

    let paths = std::fs::read_dir(format!("{}/assets/fact_check/", root_folder)).unwrap();
    let mut images: Vec<String> = vec![];

    for path in paths {
        images.push(format!("{}", path.unwrap().path().display().to_string()));
    }

    let rand_index = rand::random::<usize>() % images.len();
    let image = &images[rand_index];

    (File::open(image).await.unwrap(), image.to_string())
}


#[poise::command(prefix_command)]
pub async fn get_leaderboard(ctx: Context<'_>, param: Option<String>) -> Result<(), Error> {
    let http = ctx.http();
    let leaderboard = ctx.data().evening_leaderboard.lock().await;

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
            let user = user_id.to_user(http).await.expect("couldnt get user from id for leaderboard");
            let username = user.global_name.unwrap();

            top_10.push_str(&format!("{}. {}: {}\n", position, username, count));

            position += 1;
        }
        top_10.remove(top_10.len() - 1);

        top_10
    };
    
    debug!("leaderboard retrieved by command:\n{leaderboard_top_10}");

    let embed = CreateEmbed::new()
        .colour(Colour::from_rgb(255, 0, 124))
        .title("good evening leaderboard")
        .description(leaderboard_top_10);
    
    

    if let Some(param) = param {
        if param == "general" {
            const GENERAL_CHANNEL_ID: u64 = 1215048710074011692;
            let channel = ChannelId::new(GENERAL_CHANNEL_ID);
            
            let message = CreateMessage::new()
                .add_embed(embed);

            channel.send_message(http, message).await?;
        }
    } else {
        let message = CreateReply {
            embeds: vec![embed],
            ..Default::default()
        };

        ctx.send(message).await?;
    }

    Ok(())
}
