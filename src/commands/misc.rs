use poise::serenity_prelude::{ButtonStyle, CacheHttp, ChannelId};

use crate::{
    database::apps::insert_application,
    helpers::common::{format_dt, snowflake_time, random_component_id},
    Command, Context, Error,
};

/// Apply to use Kanzen. Kanzen is invite-only
#[poise::command(slash_command)]
pub async fn apply(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let data = ctx.data();
    let application_channel_id = data.config.staff.applications_channel_id;
    let channel = ChannelId(application_channel_id);
    let guild = &ctx.guild().unwrap();
    let member_count = guild.member_count;
    let invite = ctx
        .channel_id()
        .create_invite(&ctx.http(), |ci| {
            ci.temporary(true)
                .unique(false)
                .max_uses(3)
                .max_age(86400 * 3) // 3 days
        })
        .await?;

    let accept_id = random_component_id();
    let reject_id = random_component_id();
    channel
        .send_message(&ctx.http(), |cm| {
            cm.embed(|ce| {
                ce.title("New Application")
                    .field("Member Count", member_count, true)
                    .field(
                        "Created at",
                        format!(
                            "{} ({})",
                            format_dt(&snowflake_time(guild.id.0), Some("F")),
                            format_dt(&snowflake_time(guild.id.0), Some("R"))
                        ),
                        true,
                    )
                    .author(|cea| {
                        cea.name(&ctx.author().name).icon_url(
                            &ctx.author()
                                .avatar_url()
                                .unwrap_or(ctx.author().default_avatar_url()),
                        )
                    });
                if let Some(thumb) = guild.icon_url() {
                    ce.thumbnail(thumb);
                }
                if let Some(banner) = guild.banner_url() {
                    ce.image(banner);
                }
                ce
            });
            cm.components(|cc| {
                cc.create_action_row(|car| {
                    car.create_button(|cb| {
                        cb.label("Join").url(invite.url()).style(ButtonStyle::Link)
                    })
                    .create_button(|cb| {
                        cb.custom_id(&accept_id)
                        .label("Accept")
                        .style(ButtonStyle::Success)
                    })
                    .create_button(|cb| {
                        cb.custom_id(&reject_id)
                        .label("Reject")
                        .style(ButtonStyle::Danger)
                    })
                })
            })
        })
        .await?;

    insert_application(&data.pool, &ctx.guild_id().unwrap(), &accept_id, &reject_id).await?;
    ctx.send(|cm| {
        cm.embed(|ce| {
            ce.colour(0x8BF1AC)
            .title("Application created")
            .description(
                "Your application has been created. You should receive a response within 24 hours. 
                If not, feel free to join our support server. One of our staff members may join your server. 
                Remember, acceptance is at staff's discretion.
                Kanzen will remain invite-only until its verified, where its invite-only structure will be 
                re-evaluated. This is to avoid Discord quarantining Kanzen due to *suspicious* growth.
                "
            )
        })
    }).await?;
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [apply()]
}
