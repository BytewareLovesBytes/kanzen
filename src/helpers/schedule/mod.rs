pub mod core;
pub mod structs;

use reqwest::{Client, Error as ReqwestError};

use self::structs::AnimeObject;

const SCHEDULE_BASE_CDN_URL: &str = "https://img.animeschedule.net/production/assets/public/img/";
const SCHEDULE_API_BASE: &str = "https://animeschedule.net/api/v3";

pub async fn get_weekly_timetable(
    client: &Client,
    token: &str,
) -> Result<Vec<AnimeObject>, ReqwestError> {
    let url = format!("{SCHEDULE_API_BASE}/timetables?tz=UTC");
    let test: Vec<std::collections::HashMap<String, serde_json::Value>> = client.get(&url)
    .header("Authorization", format!("Bearer {token}"))
    .send()
    .await?
    .json()
    .await?;
    println!("{:#?}", test);
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
