use poise::serenity_prelude::{ChannelId, GuildId};

use crate::{PgError, PgPool};

pub async fn upsert_schedule_channel(
    pool: &PgPool,
    guild_id: &GuildId,
    channel_id: &ChannelId,
) -> Result<(), PgError> {
    let guild_id = guild_id.0 as i64;
    let channel_id = channel_id.0 as i64;
    sqlx::query(
        "INSERT INTO schedules(guild_id, channel_id) VALUES($1, $2) ON CONFLICT(guild_id) 
        DO UPDATE SET channel_id = $2
        ",
    )
    .bind(guild_id)
    .bind(channel_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_schedule_channels(pool: &PgPool) -> Result<Vec<(i64,)>, PgError> {
    let stream = sqlx::query_as(
        "SELECT channel_id FROM schedules 
        ",
    )
    .fetch_all(pool)
    .await?;

    Ok(stream)
}
