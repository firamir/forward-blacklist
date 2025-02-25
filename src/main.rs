use teloxide::prelude::*;
use teloxide_core::types::MessageEntityKind::CustomEmoji;
use teloxide_core::types::{MessageId, MessageKind::NewChatMembers, MessageOrigin};
use teloxide_core::{ApiError, RequestError};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let bot = Bot::from_env();
    teloxide::repl(bot, handle_message).await;
}

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let bot_id = bot.get_me().await.expect("").id;

    handle_bot_join(&bot, chat_id, &msg, bot_id).await?;

    handle_message_violations(&bot, chat_id, &msg).await?;

    Ok(())
}

async fn handle_message_violations(
    bot: &Bot,
    chat_id: ChatId,
    msg: &Message,
) -> ResponseResult<()> {
    if forward_has_blacklisted_title(msg) {
        let response = msg
            .from
            .as_ref()
            .and_then(|user| user.username.as_ref())
            .map_or_else(
                || "чювачок без хендла тут незя dvach".into(),
                |username| format!("@{username} але нельзя двач"),
            );

        delete_message(bot, chat_id, msg.id, response).await?;
        return Ok(());
    }

    if let Some(emojis) = msg.parse_caption_entities() {
        for emoji in emojis {
            if let CustomEmoji { custom_emoji_id } = emoji.kind() {
                if custom_emoji_id == "5359339614484061385" {
                    let message = msg
                        .from
                        .as_ref()
                        .and_then(|user| user.username.as_ref())
                        .map_or_else(
                            || "hello kent ). без алфавита двача. Спасибо".into(),
                            |username| format!("@{username} привет... без алфавита двача ) "),
                        );

                    delete_message(bot, chat_id, msg.id, message).await?;
                }
            }
        }
    }
    Ok(())
}
async fn handle_bot_join(
    bot: &Bot,
    chat_id: ChatId,
    msg: &Message,
    bot_user_id: UserId,
) -> ResponseResult<()> {
    if let NewChatMembers(ref members) = msg.kind {
        if members.new_chat_members.iter().any(|u| u.id == bot_user_id) {
            let inviter = msg.from.as_ref().and_then(|user| user.username.as_ref());
            let response = inviter.map_or_else(
                || "salam. deletion perms ty".into(),
                |username| format!("@{username} delet permision pls"),
            );

            bot.send_message(chat_id, response).await?;
        }
    }
    Ok(())
}

fn forward_has_blacklisted_title(msg: &Message) -> bool {
    // #TODO
    if let Some(origin) = msg.forward_origin() {
        if let MessageOrigin::Channel { chat, .. } = origin {
            if let Some(title) = chat.title() {
                return title.contains("Двач") || title == "Абу";
            }
        }
    }
    false
}

async fn delete_message(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    message: String,
) -> ResponseResult<()> {
    match bot.delete_message(chat_id, message_id).await {
        Ok(_) => {
            bot.send_message(chat_id, message).await?;
            Ok(())
        }
        Err(RequestError::Api(ApiError::MessageCantBeDeleted)) => {
            bot.send_message(
                chat_id,
                "я нимагу удалять сабщеня admin дай permissions pls thanks",
            )
            .await?;
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
