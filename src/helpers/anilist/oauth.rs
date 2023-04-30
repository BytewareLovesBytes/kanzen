use reqwest::{header::HeaderMap, Client, Error as ReqwestError};
use serde::Deserialize;
use serde_json::json;

use super::{
    perform_anilist_query,
    structs::{Response, User, ViewerData},
};

const TOKEN_URL: &str = "https://anilist.co/api/v2/oauth/token";

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
    pub token_type: String,
}

pub fn format_oauth_url(client_id: &u16, redirect_uri: &str) -> String {
    format!(
            "https://anilist.co/api/v2/oauth/authorize?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code"
        )
}
pub async fn exchange_code(
    client: &Client,
    code: &str,
    client_id: &u16,
    client_secret: &str,
    redirect_uri: &str,
) -> Result<TokenResponse, ReqwestError> {
    let payload = json!({
        "grant_type": "authorization_code",
        "client_id": client_id,
        "client_secret": client_secret,
        "redirect_uri": redirect_uri,
        "code": code
    });

    let response = client
        .post(TOKEN_URL)
        .body(payload.to_string())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
pub async fn get_authenticated_user(
    client: &Client,
    access_token: &String,
) -> Result<User, ReqwestError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", access_token).parse().unwrap(),
    );
    let query: &str = "
    query {
        Viewer {
            name,
            id,
            avatar {
                large,
                medium
            },
            bannerImage,
            siteUrl,
            about(asHtml: false),
            statistics {
                manga {
                    volumesRead,
                    chaptersRead,
                    count
                }
                anime {
                    minutesWatched,
                    episodesWatched,
                    count
                }
            }
        }
    }
    ";
    let data: Response<ViewerData> =
        perform_anilist_query(client, query, json!({}), Some(headers)).await?;
    let user = data.data.viewer;
    Ok(user)
}
