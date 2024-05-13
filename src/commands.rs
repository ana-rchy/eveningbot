use poise::{serenity_prelude::CreateAttachment, reply::CreateReply};
use tokio::fs::File;

type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn fact_check(ctx: Context<'_>) -> Result<(), Error> {
    let (image, filename) = get_fact_check_image().await;
    let attachment = CreateAttachment
        ::file(&image, filename).await.unwrap();

    let reply = CreateReply {
        attachments: vec![attachment],
        ..Default::default()
    };

    ctx.send(reply).await?;
    
    Ok(())
}


async fn get_fact_check_image() -> (File, String) {
    let root_folder = get_assets_root_path();

    let paths = std::fs::read_dir(format!("{}/assets/fact_check/", root_folder)).unwrap();
    let mut images: Vec<String> = vec![];

    for path in paths {
        images.push(format!("{}", path.unwrap().path().display().to_string()));
    }

    let rand_index = rand::random::<usize>() % images.len();
    let image = &images[rand_index];

    (File::open(image).await.unwrap(), image.to_string())
}

fn get_assets_root_path() -> String {
    let mut exec_path = std::env::current_exe().unwrap();
    exec_path.pop();
    if cfg!(debug_assertions) {
        exec_path.pop(); exec_path.pop();
    }

    exec_path.to_string_lossy().to_string()
}
