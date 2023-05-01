mod commands;
mod config;
mod database;
mod helpers;

use poise::serenity_prelude as serenity;
use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Error as SqlxError, Pool,
};

use config::Config;

pub struct Data {
    pub http: reqwest::Client,
    pub config: Config,
    pub pool: Pool<Postgres>,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;
pub type PgPool = Pool<Postgres>; // just a shorthand
pub type PgError = SqlxError;

#[tokio::main]
async fn main() {
    let conf: Config = toml::from_str(
        &std::fs::read_to_string("config.toml").expect("Could not read config.toml"),
    )
    .expect("Could not build config from file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conf.database.connection_url)
        .await
        .expect("Could not connect to database");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            ..Default::default()
        })
        .token(&conf.discord.token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId(conf.discord.test_guild_id),
                )
                .await?;
                Ok(Data {
                    http: reqwest::Client::new(),
                    config: conf,
                    pool,
                })
            })
        });

    println!("Starting bot...");
    framework.run().await.unwrap();
}
