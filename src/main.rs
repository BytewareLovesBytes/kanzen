mod commands;
mod config;
mod database;
mod helpers;

use std::sync::Arc;

use poise::serenity_prelude::{self as serenity, EventHandler, InteractionType, GuildId};
use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Error as SqlxError, Pool,
};
use tracing::info;
use async_trait::async_trait;

use crate::{helpers::schedule::core::ScheduleCore, database::apps::get_guild_application};
use config::Config;

#[derive(Clone)]
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

struct Events {
    data: Option<&'static Data>
}
#[async_trait]
impl EventHandler for Events {
    async fn interaction_create(&self, ctx: serenity::Context, interaction: serenity::Interaction) {
        if interaction.kind() == InteractionType::MessageComponent {
            let msg_c = interaction.message_component();
            let data = self.data.unwrap(); // this shouldn't panic
            match msg_c {
                Some(interaction) => {
                    let custom_id = interaction.data.custom_id;
                    match get_guild_application(&data.pool, &custom_id).await {
                        Ok((guild_id,)) => {
                            let guild = GuildId(guild_id);
                            // TODO: Complete this
                        },
                        Err(_) => {
                            // let's ignore this for now
                        }
                    }
                },
                None => {
                    info!("Message Component interaction received but failed to convert into MessageComponentInteraction")
                    // I don't know how we got here
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let conf: Config = toml::from_str(
        &std::fs::read_to_string("config.toml").expect("Could not read config.toml"),
    )
    .expect("Could not build config from file");

    let mut scheduler = ScheduleCore::new();

    let pool = PgPoolOptions::new()
        .max_connections(conf.database.max_connections)
        .connect(&conf.database.connection_url)
        .await
        .expect("Could not connect to database");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            ..Default::default()
        })
        .token(&conf.discord.token)
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILDS)
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                for guild_id in &conf.discord.test_guild_ids {
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        serenity::GuildId(guild_id.to_owned()),
                    )
                    .await?;
                }
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let data = Data {
                    http: reqwest::Client::new(),
                    config: conf,
                    pool,
                };
                let arc_ctx = Arc::new(ctx.to_owned());
                let arc_data = Arc::new(data.clone());
                let gc = ready.guilds.len();
                info!("Working inside of {gc} guilds");
                scheduler
                    .create_tasks(&data.http, &data.config.schedule.token)
                    .await;
                scheduler.start(arc_ctx, arc_data).await;
                let activity = serenity::Activity::watching("for anime releases");
                ctx.set_activity(activity).await;
                Ok(data)
            })
        });

    info!("Starting bot...");
    framework.run().await.unwrap();
}
