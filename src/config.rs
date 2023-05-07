use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Discord {
    pub token: String,
    pub test_guild_id: Vec<u64>,
}

#[derive(Deserialize, Clone)]
pub struct AniList {
    pub client_id: u16,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Deserialize, Clone)]
pub struct AnimeSchedule {
    pub token: String,
}

#[derive(Deserialize, Clone)]
pub struct Database {
    pub connection_url: String,
    pub max_connections: u32,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub discord: Discord,
    pub anilist: AniList,
    pub database: Database,
    pub schedule: AnimeSchedule,
}
