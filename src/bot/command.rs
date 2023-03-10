use teloxide::utils::command::BotCommands;
use teloxide::{
    payloads::SendMessageSetters,
    requests::{Requester, ResponseResult},
    types::Message,
    Bot,
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum TCommand {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Convert")]
    Convert(String),
    #[command(description = "Convert short hand")]
    C(String),
    #[command(description = "Dice")]
    Dice(String),
}

pub async fn answer(bot: Bot, msg: Message, cmd: TCommand) -> ResponseResult<()> {
    match cmd {
        TCommand::Help => {
            bot.send_message(msg.chat.id, TCommand::descriptions().to_string())
                .await?
        }
        TCommand::Convert(text) => convert(text, bot, msg).await?,
        TCommand::C(text) => convert(text, bot, msg).await?,
        TCommand::Dice(_) => send_dice(bot,msg).await?,
    };

    Ok(())
}

async fn send_dice(bot: Bot, msg: Message) -> Result<Message, teloxide::RequestError>  {
    Ok(bot
        .send_dice(msg.chat.id)
        .await?
    )
}

async fn convert(text: String, bot: Bot, msg: Message) -> Result<Message, teloxide::RequestError> {
    let conversion = crate::conversion::convert(text.as_str()).await;
    let res = match conversion {
        Ok(_) => conversion.unwrap(),
        Err(_) => conversion.unwrap_err(),
    };

    Ok(bot
        .send_message(msg.chat.id, res)
        .reply_to_message_id(msg.id)
        .await?)
}

