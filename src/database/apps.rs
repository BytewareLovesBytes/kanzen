use poise::serenity_prelude::GuildId;

use crate::{PgError, PgPool};

pub async fn insert_application(
    pool: &PgPool,
    guild_id: &GuildId,
    accept_id: &str,
    reject_id: &str
) -> Result<(), PgError> {
    let guild_id = guild_id.0 as i64;
    sqlx::query("INSERT INTO applications(guild_id, accept_id, reject_id) VALUES($1, $2, $3)")
        .bind(guild_id)
        .bind(accept_id)
        .bind(reject_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_guild_application(
    pool: &PgPool,
    custom_id: &str
) -> Result<(u64,), PgError> {
    let row: (i64,) = sqlx::query_as("SELECT guild_id FROM applications WHERE accept_id = $1 OR reject_id = $2")
        .bind(custom_id)
        .fetch_one(pool)
        .await?;
    let (guild_id,) = row;

    Ok((guild_id as u64,))
}