use futures::StreamExt;
use std::{
    error::Error,
    process,
    sync::{Arc, Mutex},
    time::Duration,
};
use telegram_bot::types::requests::*;
use telegram_bot::*;
use timely_greeter_bot::{chat_id, env, timezone};
use tokio::sync::mpsc;

type ShareableIds = Arc<Mutex<Vec<ChatId>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot_token = env::load_bot_token().unwrap_or_else(|err| {
        log::error!("Could not read bot token: {}", err);
        process::exit(1);
    });

    log::info!("Starting bot...");

    let api = telegram_bot::Api::new(bot_token);
    let chat_ids = Arc::new(Mutex::new(chat_id::get_stored_chat_ids().unwrap()));

    let (tx, mut rx) = mpsc::channel::<String>(2);

    tokio::task::spawn(async move {
        let mut greeter = timezone::TimezoneGreeter::new();
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            let cities_to_greet = greeter.get_cities_to_greet();
            if cities_to_greet.len() > 0 {
                let message = timezone::get_good_morning_message(&cities_to_greet);
                log::info!("{}", message);
                tx.send(message).await.unwrap_or_else(|err| {
                    log::error!("Failed to send message: {}", err);
                });
            } else {
                log::debug!("No cities to greet");
            }
        }
    });

    let notify_loop = async {
        while let Some(message) = rx.recv().await {
            notify(&api, &message, chat_ids.clone())
                .await
                .unwrap_or_else(|err| log::error!("Failed to notify users: {}", err));
        }
    };

    let bot_api = api.clone();
    let bot_loop = async {
        let mut stream = bot_api.stream();
        while let Some(update) = stream.next().await {
            if let Ok(update) = update {
                if let UpdateKind::Message(message) = update.kind {
                    if let MessageKind::Text { ref data, .. } = message.kind {
                        let username = match message.from.username {
                            Some(user) => user,
                            None => String::from("[unknown user]"),
                        };
                        log::info!("Received message from {}: \"{}\"", username, data);

                        let mut chat_ids = chat_ids.lock().unwrap();
                        let chat_id = message.chat.id();
                        if !chat_ids.contains(&chat_id) {
                            chat_ids.push(chat_id);
                            chat_id::save_chat_ids(&chat_ids).unwrap_or_else(|err| {
                                log::error!("Failed to add ID: {}", err);
                            });
                            let send_result = bot_api
                                .send(SendMessage::new(chat_id, "Hi! I'll ping you at 9 am."))
                                .await;
                            if let Err(err) = send_result {
                                log::error!("Failed to send message: {}", err);
                            }
                        }
                    }
                }
            }
        }
    };

    tokio::join!(notify_loop, bot_loop);
}

async fn notify(
    api: &telegram_bot::Api,
    message: &str,
    chat_ids: ShareableIds,
) -> Result<(), Box<dyn Error>> {
    let chat_ids = lock_and_clone_chat_ids(chat_ids);
    for chat_id in chat_ids {
        let text_message = SendMessage::new(chat_id, message);
        let text_result = api.send(text_message).await;
        if let Err(err) = text_result {
            log::error!(
                "Failed to send text message in chat ID {}: {}",
                chat_id,
                err
            );
        }
        // api.send()
        // bot.send_sticker(
        //     ChatId(chat_id),
        //     InputFile::file_id(
        //         "CAACAgIAAxkBAAMHYoAnQ-mjFlYcQI7MY6ofspGVa50AAjkBAAIQIQIQ0zO07gSDOlQkBA",
        //     ),
        // )
        // .await?;
        // bot.send_message(ChatId(chat_id), message).await?;
    }
    Ok(())
}

fn lock_and_clone_chat_ids(chat_ids: ShareableIds) -> Vec<ChatId> {
    let chat_ids = chat_ids.lock().unwrap();
    chat_ids.clone()
}
