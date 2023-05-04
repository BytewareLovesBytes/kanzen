use poise::serenity_prelude::channel::GuildChannel;

use crate::{
    database::schedule::upsert_schedule_channel, helpers::quick_embed, Command, Context, Error,
};

#[poise::command(slash_command, subcommands("setup"))]
pub async fn schedule(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn setup(
    ctx: Context<'_>,
    #[description = "Channel to post anime releases in"]
    #[channel_types("Text")]
    channel: GuildChannel,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let data = ctx.data();
    println!("CHANNEL ID: {:?} {} {}", &channel.id, &channel.id.0, &channel.id.as_u64());

    upsert_schedule_channel(&data.pool, &ctx.guild().unwrap().id, &channel.id).await?;
    quick_embed(&ctx, "Updated schedule channel successfully").await?;

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [schedule()]
}
