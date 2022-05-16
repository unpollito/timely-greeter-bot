use std::{
    collections::HashMap,
    process,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use timely_greeter_bot::{env, persist, telegram::*, timezone};

type ShareableIds = Arc<Mutex<Vec<i64>>>;

const GOOD_MORNING_STICKER_ID: &str =
    "CAACAgIAAxkBAAMHYoAnQ-mjFlYcQI7MY6ofspGVa50AAjkBAAIQIQIQ0zO07gSDOlQkBA";

fn main() {
    pretty_env_logger::init_timed();
    let token = env::load_bot_token().unwrap_or_else(|err| {
        log::error!("Could not read bot token: {}", err);
        process::exit(1);
    });
    let telegram_api_url = format!("https://api.telegram.org/bot{}", token);

    log::info!("Starting bot...");

    let chat_ids: ShareableIds = Arc::new(Mutex::new(persist::get_stored_chat_ids().unwrap()));

    let bot_loop_chat_ids = chat_ids.clone();
    let bot_loop_api_url = telegram_api_url.clone();
    thread::spawn(move || {
        let mut last_update_id = persist::get_stored_update_id().unwrap_or_else(|err| {
            log::error!("Failed to read last update ID: {}", err);
            0
        });
        let client = reqwest::blocking::Client::new();
        loop {
            thread::sleep(Duration::from_millis(250));
            let req = client
                .get(format!(
                    "{}/getUpdates?timeout=5&offset={}",
                    bot_loop_api_url,
                    last_update_id + 1,
                ))
                .send();
            if req.is_err() {
                log::warn!("Failed to get updates: {}", req.unwrap_err());
                continue;
            }
            let response: TelegramUpdatesResponse =
                req.unwrap().json::<TelegramUpdatesResponse>().unwrap();
            let mut new_update_id = last_update_id;
            for update in response.result {
                if update.update_id > new_update_id {
                    new_update_id = update.update_id;
                }
                let text = match update.message.text {
                    Some(text) => text,
                    None => String::from("[no message]"),
                };
                log::info!(
                    "Received message from {}: \"{}\"",
                    update.message.from.username,
                    text
                );

                let chat_id = update.message.chat.id;
                if text == "/stop" {
                    let mut chat_ids = bot_loop_chat_ids.lock().unwrap();
                    let index_result = chat_ids.iter().position(|&x| x == chat_id);
                    if index_result.is_some() {
                        chat_ids.remove(index_result.unwrap());
                        persist::save_chat_ids(&chat_ids).unwrap_or_else(|err| {
                            log::error!("Failed to persist chat IDs: {}", err);
                        });
                        send_message(
                            &client,
                            &bot_loop_api_url,
                            chat_id,
                            "OK, I won't greet you anymore. :'(",
                        );
                    }
                } else {
                    let mut chat_ids = bot_loop_chat_ids.lock().unwrap();
                    let index_result = chat_ids.iter().position(|&x| x == chat_id);
                    if index_result.is_some() {
                        send_message(
                            &client,
                            &bot_loop_api_url,
                            chat_id,
                            "You are already being greeted! Chill. :P",
                        );
                    } else {
                        chat_ids.push(chat_id);
                        persist::save_chat_ids(&chat_ids).unwrap_or_else(|err| {
                            log::error!("Failed to persist chat IDs: {}", err);
                        });
                        send_message(
                            &client,
                            &bot_loop_api_url,
                            chat_id,
                            "I'll greet you when it's morning. :)",
                        )
                    }
                }
            }

            if new_update_id != last_update_id {
                last_update_id = new_update_id;
                persist::save_update_id(last_update_id).unwrap_or_else(|err| {
                    log::error!("Failed to persist update ID: {}", err);
                });
            }
        }
    });

    let client = reqwest::blocking::Client::new();
    let mut greeter = timezone::TimezoneGreeter::new();
    loop {
        thread::sleep(Duration::from_secs(60));
        let cities_to_greet = greeter.get_cities_to_greet();
        if let Some(message) = timezone::get_good_morning_message(&cities_to_greet) {
            log::info!("{}", message);
            let chat_ids = lock_and_clone_chat_ids(chat_ids.clone());
            for chat_id in chat_ids {
                send_sticker(&client, &telegram_api_url, chat_id, GOOD_MORNING_STICKER_ID);
                send_message(&client, &telegram_api_url, chat_id, &message);
            }
        } else {
            log::debug!("No cities to greet");
        }
    }
}

fn lock_and_clone_chat_ids(chat_ids: ShareableIds) -> Vec<i64> {
    let chat_ids = chat_ids.lock().unwrap();
    chat_ids.clone()
}

fn send_message(
    client: &reqwest::blocking::Client,
    base_url: &str,
    chat_id: i64,
    message: &str,
) -> () {
    let mut body = HashMap::new();
    body.insert("chat_id", chat_id.to_string());
    body.insert("text", String::from(message));
    let url = format!("{}/sendMessage", base_url);
    if let Err(err) = client.post(url).json(&body).send() {
        log::error!("Failed to send message: {}", err);
    }
}

fn send_sticker(
    client: &reqwest::blocking::Client,
    base_url: &str,
    chat_id: i64,
    sticker_id: &str,
) -> () {
    let mut body = HashMap::new();
    body.insert("chat_id", chat_id.to_string());
    body.insert("sticker", String::from(sticker_id));
    let url = format!("{}/sendSticker", base_url);
    if let Err(err) = client.post(url).json(&body).send() {
        log::error!("Failed to send  {}", err);
    }
}
