use crate::command::answer;
use teloxide::prelude::*;
mod command;
mod currency;
mod distance;
mod parse;
mod temp;
mod unit;
mod volume;
mod weight;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    command::TCommand::repl(bot, answer).await;
}
