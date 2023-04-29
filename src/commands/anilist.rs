use crate::{
    helpers::{
        anilist::{
            perform_anilist_query,
            structs::{Media, Response as AniListResponse},
        },
        common::EmbedPaginator,
        constants::{ANILIST_ANIME_QUERY, ANILIST_MANGA_QUERY},
    },
    Command,
};
use crate::{Context, Error};

#[poise::command(slash_command, subcommands("anime", "manga"))]
pub async fn anilist(
    _: Context<'_>
) -> Result<(), Error> {
    Ok(())
}

/// Search for anime on the AniList platform
#[poise::command(slash_command)]
pub async fn anime(
    ctx: Context<'_>,
    #[description = "Search query"] query: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let data = ctx.data();
    let mut response: AniListResponse = perform_anilist_query(
        &data.http,
        ANILIST_ANIME_QUERY,
        serde_json::json!({ "search": query }),
    )
    .await?;

    let media = response.data.page.media.get_mut(0);

    if media.is_none() {
        ctx.say("No results").await?;
        return Ok(());
    }

    let mut paginator = EmbedPaginator::new(response.data.page.media, Media::paginator_footer);
    paginator.start(ctx).await?;

    Ok(())
}

/// Search for manga on the AniList platform
#[poise::command(slash_command)]
pub async fn manga(
    ctx: Context<'_>,
    #[description = "Search query"] query: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let data = ctx.data();
    let mut response: AniListResponse = perform_anilist_query(
        &data.http,
        ANILIST_MANGA_QUERY,
        serde_json::json!({ "search": query }),
    )
    .await?;

    let media = response.data.page.media.get_mut(0);

    if media.is_none() {
        ctx.say("No results").await?;
        return Ok(());
    }

    let mut paginator = EmbedPaginator::new(response.data.page.media, Media::paginator_footer);
    paginator.start(ctx).await?;

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [anilist()]
}
