use crate::{helpers::anilist::oauth::TokenResponse, PgError, PgPool};
use poise::serenity_prelude::UserId;

pub async fn upsert_anilist_user(
    pool: &PgPool,
    discord_id: &UserId,
    tr: &TokenResponse,
) -> Result<(), PgError> {
    let discord_id = discord_id.0 as i64;
    sqlx::query(
        "INSERT INTO anilist_users VALUES($1, $2, $3, $4) ON CONFLICT (discord_id) 
        DO UPDATE SET access_token = $2, refresh_token = $3, expires_at = $4 WHERE anilist_users.discord_id = $1",
    ) // kept getting an ambigious column error so anilist_users needed to be prepended onto it
    .bind(discord_id)
    .bind(&tr.access_token)
    .bind(&tr.refresh_token)
    .bind(tr.expires_at())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_anilist_user_token_pair(
    pool: &PgPool,
    discord_id: u64,
) -> Result<(String, String), PgError> {
    let row: (String, String) = sqlx::query_as(
        "SELECT access_token, refresh_token FROM anilist_users 
        WHERE discord_id = $1",
    )
    .bind(discord_id as i64)
    .fetch_one(pool)
    .await?;

    Ok(row)
}
