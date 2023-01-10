use teloxide::{Bot, types::Message, requests::{Requester, ResponseResult}};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum TCommand {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Convert")]
    Convert(String),
}

pub async fn answer(bot: Bot, msg: Message, cmd: TCommand) -> ResponseResult<()> {
    match cmd {
        TCommand::Help => bot.send_message(msg.chat.id, TCommand::descriptions().to_string()).await?,
        TCommand::Convert ( text ) => {
           let res = crate::parse::conversion(text);
            bot.send_message(msg.chat.id, res.await).await?
        }
    };

    Ok(())
}



