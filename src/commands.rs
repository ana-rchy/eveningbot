use poise::serenity_prelude as serenity;

type Data = ();
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn fact_check(ctx: Context<'_>, arg: String) -> Result<(), Error> {
    ctx.say("asdasd").await?;

    Ok(())
}
