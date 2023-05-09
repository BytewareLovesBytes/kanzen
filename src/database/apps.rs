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
