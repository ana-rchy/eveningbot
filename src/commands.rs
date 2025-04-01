use crate::global::*;
use log::debug;
use poise::reply::CreateReply;
use poise::serenity_prelude::{
    ChannelId, Colour, CreateAttachment, CreateEmbed, CreateMessage, UserId,
};
use tokio::fs::File;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, SharedData, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn fact_check(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Nothing ever happens.").await?;

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

#[poise::command(slash_command, prefix_command)]
pub async fn roll(
    ctx: Context<'_>,
    max: Option<i64>,
    min: Option<i64>,
    max_imaginary: Option<i64>,
    min_imaginary: Option<i64>,
) -> Result<(), Error> {
    ctx.say("Nothing ever happens.").await?;

    Ok(())
}

async fn roll_2(mut min: i64, mut max: i64) -> i64 {
    // getting the true min/max values
    let temp = min;
    min = if max < min { max } else { min };
    max = if temp > max { temp } else { max };

    let range = max - min + 1; // makes max inclusive

    let rand_num = ((rand::random::<i64>() % range) + range) % range; // uses maths modulo,
                                                                      // not modulus
    rand_num + min // min acts as offset
}

#[poise::command(slash_command, prefix_command)]
pub async fn uwuify(ctx: Context<'_>, text: Option<String>) -> Result<(), Error> {
    ctx.say("Nothing ever happens.").await?;

    Ok(())
}

#[poise::command(prefix_command)]
pub async fn get_leaderboard(ctx: Context<'_>, param: Option<String>) -> Result<(), Error> {
    ctx.say("Nothing ever happens.").await?;

    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    required_bot_permissions = "BAN_MEMBERS"
)]
pub async fn roll_ban(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Nothing ever happens.").await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn say(ctx: Context<'_>, channel_id: u64, text: String) -> Result<(), Error> {
    const SERVER_ID: u64 = 1215048710074011689;
    const COMMITTEE_ROLE_ID: u64 = 1215639995302543430;

    let http = ctx.http();

    if !ctx
        .author()
        .has_role(http, SERVER_ID, COMMITTEE_ROLE_ID)
        .await
        .expect("couldnt get users roles")
    {
        return Ok(());
    }

    let channel = ChannelId::new(channel_id);

    let message = CreateMessage::new().content(text);

    channel
        .send_message(http, message)
        .await
        .expect("couldnt send say command message");

    Ok(())
}
