use reqwest::{Client, Error as ReqwestError};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

const ANILIST_BASE_URL: &'static str = "https://graphql.anilist.co";

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
    use serde::Deserialize;

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
        pub media: Vec<Media>,
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
    }
}
