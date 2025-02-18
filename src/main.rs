use teloxide::prelude::*;
use teloxide_core::types::{MessageKind::NewChatMembers, MessageNewChatMembers, MessageOrigin};
use teloxide_core::{ApiError, RequestError};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let bot = Bot::from_env();
    teloxide::repl(bot, handle_message).await;
}

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    if let NewChatMembers(MessageNewChatMembers { new_chat_members}) = &msg.kind {
        for member in new_chat_members {
            if member.id == bot.get_me().await?.id {
                bot.send_message(chat_id, format!("assalamu aleykum everynyan don't forget to give me deletion perms @{}", msg.from.unwrap().username.unwrap())).await?;
                return Ok(());
            }
        }
    }

    if let Some(MessageOrigin::Channel {chat, .. }) = msg.forward_origin() {
        let title = chat.title().unwrap();

        if !(title.contains("Двач") || title == "Абу") { return Ok(()); }

        let warn_message = match msg.from.and_then(|user| user.username) {
            Some(username) => format!("@{} але нельзя двач", username),
            None => "чювачок без хендла тут незя dvach".to_string(),
        };

        return match bot.delete_message(chat_id, msg.id).await {
            Ok(_) => {
                bot.send_message(chat_id, warn_message).await?;
                Ok(())
            }
            Err(RequestError::Api(ApiError::MessageCantBeDeleted)) => {
                bot.send_message(chat_id, "я нимагу удалять сабщеня admin дай permissions pls thanks").await?;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }

    }
    Ok(())
}