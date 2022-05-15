use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};
use teloxide::{
    prelude::*,
    types::{ChatId, InputFile},
};
use timely_greeter_bot::{chat_id, timezone};
use tokio::sync::mpsc;

type ShareableIds = Arc<Mutex<Vec<i64>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();
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
            notify(&bot, &message, chat_ids.clone())
                .await
                .unwrap_or_else(|err| log::error!("Failed to notify users: {}", err));
        }
    };

    let bot_loop = teloxide::repl(
        bot.clone(),
        |message: Message, _bot: AutoSend<Bot>| async move {
            let text = match message.text() {
                Some(text) => text,
                None => "[no message]",
            };
            let username = match message.chat.username() {
                Some(user) => user,
                None => "[unknown]",
            };
            log::info!("Received message \"{}\" from user {}", text, username);

            // let mut chat_ids = chat_ids.lock().unwrap();
            // let chat_id = chat_id::chat_id_to_i64(message.chat.id);
            // if !chat_ids.contains(&chat_id) {
            //     chat_ids.push(chat_id);
            //     chat_id::save_chat_ids(&chat_ids).unwrap_or_else(|err| {
            //         log::error!("Failed to add ID: {}", err);
            //     });
            // }
            respond(())
        },
    );

    tokio::join!(notify_loop, bot_loop);
}

async fn notify(
    bot: &AutoSend<Bot>,
    message: &str,
    chat_ids: ShareableIds,
) -> Result<(), Box<dyn Error>> {
    let chat_ids = lock_and_clone_chat_ids(chat_ids);
    for chat_id in chat_ids {
        bot.send_sticker(
            ChatId(chat_id),
            InputFile::file_id(
                "CAACAgIAAxkBAAMHYoAnQ-mjFlYcQI7MY6ofspGVa50AAjkBAAIQIQIQ0zO07gSDOlQkBA",
            ),
        )
        .await?;
        bot.send_message(ChatId(chat_id), message).await?;
    }
    Ok(())
}

fn lock_and_clone_chat_ids(chat_ids: ShareableIds) -> Vec<i64> {
    let chat_ids = chat_ids.lock().unwrap();
    chat_ids.clone()
}
