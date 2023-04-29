use reqwest::{Client, Error as ReqwestError};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

const ANILIST_BASE_URL: &str = "https://graphql.anilist.co";

pub async fn perform_anilist_query<T: DeserializeOwned>(
    client: &Client,
    query: &str,
    variables: Value,
) -> Result<T, ReqwestError> {
    let body = json!({
        "query": query,
        "variables": variables
    });
    let response = client
        .post(ANILIST_BASE_URL)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(body.to_string())
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

pub mod structs {
    use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
    use serde::Deserialize;
    use std::collections::VecDeque;

    use crate::helpers::{common::ToEmbed, constants::ANILIST_ICON};

    #[derive(Deserialize, Debug)]
    pub struct Response {
        pub data: ResponseData,
    }

    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "Page")]
        pub page: Page,
    }

    #[derive(Deserialize, Debug)]
    pub struct Page {
        #[serde(rename = "pageInfo")]
        pub page_info: PageInfo,
        pub media: VecDeque<Media>,
    }

    #[derive(Deserialize, Debug)]
    pub struct PageInfo {
        pub total: u32,
        #[serde(rename = "currentPage")]
        pub current_page: u32,
        #[serde(rename = "lastPage")]
        pub last_page: u32,
    }

    #[derive(Deserialize, Debug)]
    pub struct Title {
        pub romaji: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Media {
        pub title: Title,
        pub description: String,
        #[serde(rename = "siteUrl")]
        pub site_url: String,
        #[serde(rename = "bannerImage")]
        pub banner_image: Option<String>,
        #[serde(rename = "coverImage")]
        pub cover_image: Image,
    }

    #[derive(Deserialize, Debug)]
    pub struct Image {
        pub large: Option<String>,
        pub medium: Option<String>,
        pub small: Option<String>,
    }

    impl Image {
        pub fn first(&self) -> Option<&String> {
            self.large
                .as_ref()
                .or(self.medium.as_ref().or(self.small.as_ref()))
        }
    }

    impl Media {
        /// Cleaned description
        pub fn clean_description(&mut self) {
            let to_replace = vec!["<br>", "<i>", "</i>"];
            for tr in to_replace {
                self.description = self.description.replace(tr, "");
            }
        }
        pub fn paginator_footer(cf: &mut CreateEmbedFooter, current: usize, last: usize) {
            cf.text(format!("AniList - {current}/{last}"))
                .icon_url(ANILIST_ICON);
        }
    }

    impl ToEmbed for Media {
        fn to_embed(&mut self, ce: &mut CreateEmbed) {
            self.clean_description();
            ce.colour(0x009AFF)
                .title(&self.title.romaji)
                .description(&self.description)
                .url(&self.site_url);

            if let Some(banner_image) = &self.banner_image {
                ce.image(banner_image);
            }
            if let Some(cover_image) = &self.cover_image.first() {
                ce.thumbnail(cover_image);
            }
        }
    }
}
