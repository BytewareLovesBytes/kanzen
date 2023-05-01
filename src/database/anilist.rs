use crate::{helpers::anilist::oauth::TokenResponse, PgError, PgPool};

pub async fn upsert_anilist_user(
    pool: &PgPool,
    discord_id: u64,
    tr: &TokenResponse,
) -> Result<(), PgError> {
    sqlx::query(
        "INSERT INTO anilist_users VALUES($1, $2, $3, $4) ON CONFLICT (discord_id) 
        DO UPDATE SET access_token = $2, refresh_token = $3, expires_at = $4 WHERE anilist_users.discord_id = $1",
    ) // kept getting an ambigious column error so anilist_users needed to be prepended onto it
    .bind(discord_id as i64)
    .bind(&tr.access_token)
    .bind(&tr.refresh_token)
    .bind(tr.expires_at())
    .execute(pool)
    .await?;
    Ok(())
}
