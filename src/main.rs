mod commands;
mod config;
mod helpers;

use poise::serenity_prelude as serenity;

use config::Config;

pub struct Data {
    pub http: reqwest::Client,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let conf: Config = toml::from_str(
        &std::fs::read_to_string("config.toml").expect("Could not read config.toml"),
    )
    .expect("Could not build config from file");

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
                })
            })
        });

    println!("Starting bot...");
    framework.run().await.unwrap();
}
