mod bot;
mod conversion;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot bot...");

    let bot = Bot::from_env();

    bot::command::TCommand::repl(bot, bot::command::answer).await;
}
