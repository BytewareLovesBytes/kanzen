use serde::Deserialize;

#[derive(Deserialize)]
pub struct Discord {
    pub token: String,
    pub test_guild_id: u64,
}

#[derive(Deserialize)]
pub struct Config {
    pub discord: Discord,
}
