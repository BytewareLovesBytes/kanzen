use crate::{
    helpers::{
        anilist::{perform_anilist_query, structs::Response as AniListResponse},
        constants::ANILIST_ANIME_QUERY,
    },
    Command,
};
use crate::{Context, Error};

/// Search for anime on the AniList platform
#[poise::command(slash_command)]
pub async fn search(
    ctx: Context<'_>,
    #[description = "Search query"] query: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let data = ctx.data();
    let response: AniListResponse = perform_anilist_query(
        &data.http,
        ANILIST_ANIME_QUERY,
        serde_json::json!({ "search": query }),
    )
    .await?;

    println!("{response:#?}");

    let media = response.data.page.media.get(0).unwrap();

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.description(&media.description)
                .title(&media.title.romaji)
                .colour(0x009AFF)
                .url(&media.site_url);

            if let Some(banner_image) = &media.banner_image {
                ce.image(banner_image);
            }
            ce
        })
    })
    .await?;

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [search()]
}
