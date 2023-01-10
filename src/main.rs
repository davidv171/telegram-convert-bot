use teloxide::prelude::*;
use crate::command::answer;
mod command;
mod parse;
mod unit;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    command::TCommand::repl(bot, answer).await;
}
