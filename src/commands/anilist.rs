use poise::serenity_prelude::{ActionRowComponent, ButtonStyle, CacheHttp, EmojiId, ReactionType};

use crate::{
    helpers::{
        anilist::{
            oauth::format_oauth_url,
            perform_anilist_query,
            structs::{Media, Response as AniListResponse},
        },
        common::EmbedPaginator,
        constants::{ANILIST_ANIME_QUERY, ANILIST_ICON, ANILIST_MANGA_QUERY},
        random_component_id,
    },
    Command,
};
use crate::{Context, Error};

#[poise::command(slash_command, subcommands("anime", "manga", "link"))]
pub async fn anilist(_: Context<'_>) -> Result<(), Error> {
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
        None,
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
        None,
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

/// Link your AniList account
#[poise::command(slash_command)]
pub async fn link(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let anilist_conf = &data.config.anilist;
    let oauth_url = format_oauth_url(&anilist_conf.client_id, &anilist_conf.redirect_url);

    let emoji = ReactionType::Custom {
        animated: false,
        id: EmojiId(1102187833155932190),
        name: Some("anilist".to_string()),
    };

    let code_btn = random_component_id();
    let modal_id = random_component_id();
    let modal_input_id = random_component_id();

    let handle = ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Link AniList Account")
            .thumbnail(ANILIST_ICON)
            .description("
            Click on the button below. It'll redirect you to the AniList website where you will then be prompted 
            to authorize us. Don't worry, this process is 100% secure.\n\nYou will then be redirected to our site. Copy the code 
            our site gives you and use the second button below to paste it. After this, you should receive a confirmation message 
            from our bot, and your AniList account will be linked.
            ")
            .colour(0x009AFF)
        });
        cr.components(|cc| {
            cc.create_action_row(|car| {
                car.create_button(|cb| {
                    cb.label("AniList")
                    .style(ButtonStyle::Link)
                    .url(oauth_url)
                    .emoji(emoji)
                });
                car.create_button(|cb| {
                    cb.label("Enter Code")
                    .custom_id(&code_btn)
                    .style(ButtonStyle::Success)
                })
            })
        });
        cr.ephemeral(true)
    }).await?;

    let message = handle.message().await?;
    let button_interaction = message
        .await_component_interaction(&ctx.serenity_context().shard)
        .author_id(ctx.author().id)
        .timeout(std::time::Duration::from_secs(60 * 2))
        .await;

    if let Some(btn_interaction) = button_interaction {
        btn_interaction.create_interaction_response(ctx.http(), |cir| {
            cir.kind(poise::serenity_prelude::InteractionResponseType::Modal);
            cir.interaction_response_data(|d| {
                d.title("Enter Code")
                .custom_id(&modal_id)
                .components(|c| {
                    c.create_action_row(|car| {
                        car.create_input_text(|cit| {
                            cit.label("AniList Code")
                            .style(poise::serenity_prelude::InputTextStyle::Paragraph)
                            .required(true)
                            .custom_id(&modal_input_id)
                            .placeholder("The AniList code you copied from our site. Yes, its a massive code.")
                        })
                    })
                })
            })
        }).await?;
        let modal_interaction = message
            .await_modal_interaction(&ctx.serenity_context().shard)
            .await;
        if let Some(modal_interaction) = modal_interaction {
            let row1 = modal_interaction.data.components.get(0).unwrap();
            let text_component = row1.components.get(0).unwrap();
            match text_component {
                ActionRowComponent::InputText(text) => {
                    modal_interaction.defer(ctx.http()).await?;
                    println!("{}", text.value);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [anilist()]
}
