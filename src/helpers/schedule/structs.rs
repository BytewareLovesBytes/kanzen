use std::str::FromStr;

use serde::Deserialize;

use super::SCHEDULE_BASE_CDN_URL;
use crate::helpers::common::{format_dt, ToEmbed};

#[derive(Deserialize, Clone, Debug)]
pub struct AnimeObject {
    pub title: String,
    pub route: String,
    #[serde(rename = "episodeNumber")]
    pub episode_number: u16,
    #[serde(rename = "lengthMin")]
    pub length_min: Option<u32>,
    #[serde(rename = "imageVersionRoute")]
    pub image_version_route: String,
    #[serde(rename = "episodeDate")]
    pub episode_date: String,
}

impl PartialEq for AnimeObject {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route
    }
}

impl ToEmbed for AnimeObject {
    fn to_embed(&mut self, ce: &mut poise::serenity_prelude::CreateEmbed) {
        let timestamp = format_dt(&self.episode_date_chrono());
        ce.title(&self.title)
            .description(format!(
                "{} was just released, go check it out",
                &self.title
            ))
            .colour(0x3D77C7)
            .field("Episode Number", &self.episode_number, true)
            .field("Released", timestamp, true)
            .thumbnail(&self.image_url());
    }
}

impl AnimeObject {
    pub fn image_url(&self) -> String {
        format!("{}{}", SCHEDULE_BASE_CDN_URL, &self.image_version_route)
    }
    pub fn episode_date_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_str(&self.episode_date).unwrap() // I don't know
                                                                // what the best way is to handle an invalid datetime at the moment
    }
}
