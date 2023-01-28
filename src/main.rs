use crate::command::answer;
use teloxide::prelude::*;
mod command;
mod parse;
mod unit;
mod currency;
mod temp;
mod distance;
mod weight;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    command::TCommand::repl(bot, answer).await;
}
