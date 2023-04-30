pub mod oauth;
pub mod structs;

use reqwest::{header::HeaderMap, Client, Error as ReqwestError};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

const ANILIST_BASE_URL: &str = "https://graphql.anilist.co";

pub async fn perform_anilist_query<T: DeserializeOwned>(
    client: &Client,
    query: &str,
    variables: Value,
    headers: Option<HeaderMap>,
) -> Result<T, ReqwestError> {
    let body = json!({
        "query": query,
        "variables": variables
    });
    let response = client
        .post(ANILIST_BASE_URL)
        .headers(headers.unwrap_or_default())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(body.to_string())
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
