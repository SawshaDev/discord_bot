use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(prefix_command)]
pub async fn hi(ctx: Context<'_>) -> Result<(), Error>{
    let mut embed = serenity::builder::CreateEmbed::default();

    embed.description("hi oowo");

    ctx.send(|m| m.embed(|e| {*e = embed; e})).await?;

    Ok(())
}

