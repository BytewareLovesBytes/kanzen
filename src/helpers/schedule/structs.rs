use std::str::FromStr;

use serde::Deserialize;

const SCHEDULE_BASE_URL: &str = "https://img.animeschedule.net/production/assets/public/img/";

#[derive(Deserialize)]
pub struct AnimeObject {
    pub title: String,
    pub route: String,
    #[serde(rename = "episodeNumber")]
    pub episode_number: u16,
    #[serde(rename = "lengthMin")]
    pub length_min: u32,
    #[serde(rename = "imageVersionRoute")]
    pub image_version_route: String,
    #[serde(rename = "episodeDate")]
    pub episode_date: String
}

impl AnimeObject {
    pub fn image_url(&self) -> String {
        format!("{}{}", SCHEDULE_BASE_URL, &self.image_version_route)
    }
    pub fn episode_date_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_str(&self.episode_date).unwrap() // I don't know
        // what the best way is to handle an invalid datetime at the moment
    }
}