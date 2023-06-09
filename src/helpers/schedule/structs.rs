use std::{collections::HashMap, str::FromStr};

use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateComponents};
use serde::Deserialize;

use super::SCHEDULE_BASE_CDN_URL;
use crate::helpers::common::{format_dt, format_title, AddComponents, ToEmbed};

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
    #[serde(rename = "airType")]
    pub air_type: String,
    pub streams: HashMap<String, String>,
}

impl PartialEq for AnimeObject {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.route
    }
}

impl ToEmbed for AnimeObject {
    fn to_embed(&mut self, ce: &mut poise::serenity_prelude::CreateEmbed) {
        let long_timestamp = format_dt(&self.episode_date_chrono(), Some("F"));
        let relative_timestamp = format_dt(&self.episode_date_chrono(), Some("R"));
        ce.title(&self.title)
            .description(format!(
                "{} was just released, go check it out",
                &self.title
            ))
            .colour(0x3D77C7)
            .field("Episode Number", &self.episode_number, true)
            .field(
                "Released",
                format!("{long_timestamp} ({relative_timestamp})"),
                true,
            )
            .field("Air Type", format_title(&self.air_type), true)
            .thumbnail(&self.image_url())
            .url(&self.site_url());
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
    pub fn site_url(&self) -> String {
        format!("https://animeschedule.net/anime/{}", self.route)
    }
}

impl AddComponents for AnimeObject {
    fn add_components(&mut self, cc: &mut CreateComponents) {
        if self.streams.len() > 0 {
            let mut row = CreateActionRow::default();
            for (title, url) in self.streams.iter() {
                let new_title = format_title(title);

                row.create_button(|cb| {
                    cb.label(&new_title)
                        .style(ButtonStyle::Link)
                        .url(format!("https://{url}"))
                });
            }
            cc.add_action_row(row);
        }
    }
}
