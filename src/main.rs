mod bot;
mod conversion;

use crate::conversion::conversion_cache::CONVERSION_CACHE;
use std::time::Duration;
use teloxide::prelude::*;
use tokio::{task, time}; // 1.3.0

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot bot...");

    let bot = Bot::from_env();

    let forever = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(3600 * 12));
        loop {
            println!("Scheduled cache update time");
            interval.tick().await;
            task::spawn_blocking(move || {
                CONVERSION_CACHE.lock().unwrap().populate().unwrap();
            })
            .await
            .unwrap();
        }
    });
    println!("Done spawning tasks");
    task::spawn(forever);

    bot::command::TCommand::repl(bot, bot::command::answer).await;
}
