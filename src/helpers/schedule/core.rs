use std::sync::{Arc, Mutex};

use poise::serenity_prelude::{CacheHttp, ChannelId, Context};
use reqwest::Client;
use tokio::{
    sync::{broadcast, Mutex as TokioMutex},
    task::JoinHandle,
};
use tracing::{debug, info};

use super::get_weekly_timetable;
use super::structs::AnimeObject;
use crate::{database::schedule::get_schedule_channels, helpers::common::ToEmbed, Data};

pub struct ScheduleCore {
    tx: Arc<Mutex<broadcast::Sender<AnimeObject>>>,
    rx: Arc<TokioMutex<broadcast::Receiver<AnimeObject>>>,
    tasks: Vec<JoinHandle<()>>,
}

impl ScheduleCore {
    pub fn new() -> Self {
        let (tx, rx) = broadcast::channel(300);
        Self {
            tx: Arc::new(Mutex::new(tx)),
            rx: Arc::new(TokioMutex::new(rx)),
            tasks: Vec::new(),
        }
    }
    pub async fn create_tasks(&mut self, client: &Client, token: &String) {
        // fetch the timetable and build the tasks
        // TODO: handle these errors
        let timetable = get_weekly_timetable(client, token).await.unwrap();
        let current = chrono::Utc::now();
        for anime in timetable {
            // check if the release time is still in the future
            // if it is, create the task
            let expected = anime.episode_date_chrono();
            if current < expected {
                debug!("Anime soon to be posted");
                // calculate sleep time
                let sleep_chrono = expected - current;
                let sleep = std::time::Duration::from_secs(sleep_chrono.num_seconds() as u64);
                let tx = Arc::clone(&self.tx);
                // build task and store it
                let handle = tokio::spawn(async move {
                    tokio::time::sleep(sleep).await;
                    // its now time to send a message to our receiver
                    // this tells the receiver to do the following:
                    // 1) check if the episode was delayed, and post in
                    // premium guilds that it was delayed
                    // 2) if it wasn't delayed, post in every guild that
                    // the episode was released
                    // 3) If it was delayed originally, spawn a new task for the
                    // delayed time
                    tx.lock().unwrap().send(anime).unwrap();
                });
                self.tasks.push(handle);
            }
        }
    }
    pub async fn handle_anime_post_time(
        anime: &mut AnimeObject,
        ctx: &Arc<Context>,
        data: &Arc<Data>,
    ) {
        // runs whenever its time to post an anime
        // TODO: check for release delays
        let name = &anime.title;
        info!("Posting anime {name} in schedule channels");
        let rows = get_schedule_channels(&data.pool).await.unwrap();
        for row in rows {
            let (channel_id,) = row;
            ChannelId(channel_id as u64)
                .send_message(&ctx.http(), |cm| {
                    cm.embed(|ce| {
                        anime.to_embed(ce);
                        ce
                    })
                })
                .await
                .unwrap();
            debug!("Posted RELEASE message in schedule channel {channel_id}");
        }
    }
    pub async fn start(&self, ctx: Arc<Context>, data: Arc<Data>) {
        let rx = Arc::clone(&self.rx);
        tokio::spawn(async move {
            loop {
                let mut anime = rx.lock().await.recv().await.unwrap();
                Self::handle_anime_post_time(&mut anime, &ctx, &data).await;
            }
        });
        info!("Started scheduler");
    }
}
