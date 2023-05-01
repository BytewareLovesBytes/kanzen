use poise::serenity_prelude::channel::GuildChannel;

use crate::{Context, Error, Command, helpers::quick_embed, database::schedule::upsert_schedule_channel};

#[poise::command(slash_command, subcommands("setup"))]
pub async fn schedule(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn setup(
    ctx: Context<'_>,
    #[description = "Channel to post anime releases in"]
    #[channel_types("Text")]
    channel: GuildChannel
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let data = ctx.data();

    upsert_schedule_channel(&data.pool, &ctx.guild().unwrap().id, &channel.id).await?;
    quick_embed(&ctx, "Updated schedule channel successfully").await?;
    
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [schedule()]
}