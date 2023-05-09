use std::collections::VecDeque;

use chrono::{DateTime, TimeZone};
use futures::StreamExt;
use poise::serenity_prelude::{
    ButtonStyle, CacheHttp, CreateActionRow, CreateComponents, CreateEmbed, CreateEmbedFooter,
};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::{Context, Error};

pub fn random_component_id() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    rand_string
}

pub async fn quick_embed(ctx: &Context<'_>, text: &str) -> Result<(), Error> {
    ctx.send(|cr| cr.embed(|ce| ce.colour(0x83BEE5).description(text)))
        .await?;

    Ok(())
}

pub trait ToEmbed {
    fn to_embed(&mut self, ce: &mut CreateEmbed);
}
pub trait AddComponents {
    fn add_components(&mut self, _ce: &mut CreateComponents) {}
    fn add_components_to_action_row(&mut self, _row: &mut CreateActionRow) {}
}

pub struct EmbedPaginator<T: ToEmbed + AddComponents> {
    items: VecDeque<T>,
    footer_formatter: fn(&mut CreateEmbedFooter, usize, usize),
}

impl<T: ToEmbed + AddComponents> EmbedPaginator<T> {
    pub fn new(
        items: VecDeque<T>,
        footer_formatter: fn(&mut CreateEmbedFooter, usize, usize),
    ) -> Self {
        Self {
            items,
            footer_formatter,
        }
    }
    pub async fn start(&mut self, ctx: Context<'_>) -> Result<(), Error> {
        // get the first item and rotate
        let mut current_idx = 0;
        let last_idx = self.items.len() - 1;

        let media = self.items.get_mut(0).unwrap();

        let next_btn_id = random_component_id();
        let handle = ctx
            .send(|cr| {
                cr.embed(|ce| {
                    media.to_embed(ce);
                    ce.footer(|cf| {
                        (self.footer_formatter)(cf, current_idx + 1, last_idx + 1);
                        cf
                    })
                });
                cr.components(|cc| {
                    cc.create_action_row(|car| {
                        car.create_button(|cb| cb.label("Next").custom_id(&next_btn_id));
                        car.create_button(|cb| {
                            cb.label("Close")
                                .custom_id(random_component_id())
                                .style(ButtonStyle::Danger)
                        });
                        media.add_components_to_action_row(car);
                        car
                    })
                })
            })
            .await?;
        let message = handle.message().await?;
        let mut collector = message
            .await_component_interactions(&ctx.serenity_context().shard)
            .channel_id(ctx.channel_id())
            .author_id(ctx.author().id)
            .message_id(message.id)
            .timeout(std::time::Duration::from_secs(60 * 5))
            .build();

        while let Some(interaction) = collector.next().await {
            if &interaction.data.custom_id == &next_btn_id {
                // next button
                // rotate and get next item
                self.items.rotate_left(1);
                let next_media = self.items.get_mut(0).unwrap();

                if current_idx == last_idx {
                    current_idx = 0
                } else {
                    current_idx += 1;
                }

                handle
                    .edit(ctx, |edit| {
                        edit.embed(|ce| {
                            next_media.to_embed(ce);
                            ce.footer(|cf| {
                                (self.footer_formatter)(cf, current_idx + 1, last_idx + 1);
                                cf
                            });
                            ce
                        });
                        edit.components(|cc| {
                            cc.create_action_row(|car| {
                                car.create_button(|cb| cb.label("Next").custom_id(&next_btn_id));
                                car.create_button(|cb| {
                                    cb.label("Close")
                                        .style(ButtonStyle::Danger)
                                        .custom_id(random_component_id())
                                });
                                next_media.add_components_to_action_row(car);
                                car
                            })
                        })
                    })
                    .await?;
                interaction.defer(&ctx.http()).await?;
            } else {
                // close button
                // stop the stream
                collector.stop();
                break;
            }
        }

        // remove the message components
        // once the stream is complete
        let last_media = self.items.get_mut(0).unwrap();
        handle
            .edit(ctx, |edit| {
                edit.components(|cc| {
                    cc.0.clear();
                    cc
                });
                edit.embed(|ce| {
                    last_media.to_embed(ce);
                    ce
                })
            })
            .await?;

        Ok(())
    }
}

pub fn format_dt<T: TimeZone>(dt: &DateTime<T>, style: Option<&str>) -> String {
    let timestamp = dt.timestamp();
    let style = style.unwrap_or("f");
    format!("<t:{timestamp}:{style}>")
}

const DISCORD_EPOCH: u64 = 1420070400000;

pub fn snowflake_time(id: u64) -> chrono::DateTime<chrono::Utc> {
    let timestamp = (id >> 22) + DISCORD_EPOCH;
    let time = chrono::Utc.timestamp_millis_opt(timestamp as i64).unwrap();
    time
}

pub fn format_title(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars[0] = chars[0].to_uppercase().nth(0).unwrap();
    let new_title: String = chars.into_iter().collect();
    new_title
}
