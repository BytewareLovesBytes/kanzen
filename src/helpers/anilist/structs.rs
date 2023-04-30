use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use serde::Deserialize;
use std::collections::VecDeque;

use crate::helpers::{common::ToEmbed, constants::ANILIST_ICON};

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct PageData {
    #[serde(rename = "Page")]
    pub page: Page,
}

#[derive(Deserialize, Debug)]
pub struct ViewerData {
    #[serde(rename = "Viewer")]
    pub viewer: User,
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

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String,
    #[serde(rename = "avatar")]
    pub avatar_image: Image,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    pub about: Option<String>,
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    pub statistics: Statistics,
}

#[derive(Deserialize, Debug)]
pub struct Statistics {
    pub anime: UserStatistics,
    pub manga: UserStatistics,
}

#[derive(Deserialize, Debug)]
pub struct UserStatistics {
    pub count: u16,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: Option<u16>,
    #[serde(rename = "episodesWatched")]
    pub episodes_watched: Option<u16>,
    #[serde(rename = "volumesRead")]
    pub volumes_read: Option<u16>,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: Option<u16>,
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

impl ToEmbed for User {
    fn to_embed(&mut self, ce: &mut CreateEmbed) {
        ce.colour(0x009AFF)
            .title(&self.name)
            .author(|ca| ca.icon_url(ANILIST_ICON).name("AniList User"))
            .url(&self.site_url);

        // unwrap_or gave me an error
        if let Some(description) = &self.about {
            ce.description(description);
        } else {
            ce.description("*No about me*");
        }

        if let Some(cover_image) = &self.avatar_image.first() {
            ce.thumbnail(cover_image);
        }

        if let Some(image) = &self.banner_image {
            ce.image(image);
        }

        let stats = &self.statistics;

        // anime formatting
        let anime = &stats.anime;
        let minutes_watched = &anime.minutes_watched.unwrap_or_default();
        let episodes_watched = &anime.episodes_watched.unwrap_or_default();
        let watch_count = &anime.count;
        ce.field(
            "Anime",
            format!("Watch Count: {watch_count}\nEpisodes Watched: {episodes_watched}\nMinutes Watched: {minutes_watched}"),
            true,
        );

        // manga formatting
        let manga = &stats.manga;
        let volumes_read = &manga.volumes_read.unwrap_or_default();
        let chapters_read = &manga.chapters_read.unwrap_or_default();
        let read_count = &manga.count;
        ce.field(
            "Manga",
            format!("Read Count: {read_count}\nChapters Read: {chapters_read}\nVolumes Read: {volumes_read}"),
            true,
        );
    }
}
