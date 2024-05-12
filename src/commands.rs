use poise::{serenity_prelude::CreateAttachment, reply::CreateReply};
use tokio::fs::File;

type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn fact_check(ctx: Context<'_>) -> Result<(), Error> {
    let image = get_fact_check_image().await;
    let attachment = CreateAttachment
        ::file(&image, "fact_check.png").await.unwrap();

    let reply = CreateReply {
        attachments: vec![attachment],
        ..Default::default()
    };

    ctx.send(reply).await?;
    
    Ok(())
}


async fn get_fact_check_image() -> File {
    let paths = std::fs::read_dir("assets/fact_check/").unwrap();
    let mut images: Vec<String> = vec![];
    for path in paths {
        images.push(path.unwrap().path().display().to_string());
    }

    let rand_index = rand::random::<usize>() % images.len();
    let image = &images[rand_index];

    File::open(image).await.unwrap()
}
