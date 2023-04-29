use std::collections::VecDeque;

use futures::StreamExt;
use poise::serenity_prelude::{ButtonStyle, CacheHttp, CreateEmbed, CreateEmbedFooter};
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

pub trait ToEmbed {
    fn to_embed(&mut self, ce: &mut CreateEmbed);
}

pub struct EmbedPaginator<T: ToEmbed> {
    items: VecDeque<T>,
    footer_formatter: fn(&mut CreateEmbedFooter, usize, usize),
}

impl<T: ToEmbed> EmbedPaginator<T> {
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
                        })
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
