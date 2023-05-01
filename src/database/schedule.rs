use poise::serenity_prelude::{GuildId, ChannelId};

use crate::{PgPool, PgError};

pub async fn upsert_schedule_channel(pool: &PgPool, guild_id: &GuildId, channel_id: &ChannelId) -> Result<(), PgError> {
    let guild_id = guild_id.0 as f64;
    let channel_id = channel_id.0 as f64;
    sqlx::query(
        "INSERT INTO schedules VALUES($1, $2) ON CONFLICT(guild_id) 
        DO UPDATE SET channel_id = $2
        "
    ).bind(guild_id)
    .bind(channel_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_schedule_channel(pool: &PgPool, guild_id: &GuildId) -> Result<ChannelId, PgError> {
    let guild_id = guild_id.0 as f64;
    let row: (f64,) = sqlx::query_as(
        "SELECT channel_id FROM schedules 
        WHERE guild_id = $1
        "
    ).bind(guild_id)
    .fetch_one(pool)
    .await?;
    let (channel_id,) = row;
    Ok(ChannelId(channel_id as u64))
}