use reqwest::{header::HeaderMap, Client, Error as ReqwestError};
use serde::Deserialize;
use serde_json::json;

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
pub async fn get_authenticated_user(client: &Client, access_token: &String) {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", access_token).parse().unwrap(),
    );
    let grapql: &str = "
        query() {
            Viewer
        }
        ";
}
