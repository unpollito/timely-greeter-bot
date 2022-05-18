use std::{collections::HashMap, error::Error};

use reqwest::Response;

use self::types::TelegramUpdatesResponse;

use super::{persist, ShareableIds};

pub mod types;

pub async fn run_bot_loop(chat_ids: &ShareableIds, bot_token: &str) -> Result<(), Box<dyn Error>> {
    let chat_ids = chat_ids.clone();
    let bot_token = String::from(bot_token);

    let mut last_update_id = persist::get_stored_update_id().unwrap_or_else(|err| {
        log::error!("Failed to read last update ID: {}", err);
        0
    });
    let client = reqwest::Client::new();
    loop {
        let response = get_updates(&client, &bot_token, last_update_id).await;
        if response.is_err() {
            log::warn!("Failed to get updates: {}", response.unwrap_err());
            continue;
        }
        let response = response.unwrap();
        let mut new_update_id = last_update_id;
        for update in response.result {
            if update.update_id > new_update_id {
                new_update_id = update.update_id;
            }
            let text = match update.message.text {
                Some(text) => text,
                None => String::from("[no message]"),
            };
            let chat_id = update.message.chat.id;
            log::info!(
                "Received message from {} (chat ID: {}): \"{}\"",
                update.message.from.username,
                chat_id,
                text
            );

            if text == "/stop" {
                let mut chat_ids = chat_ids.lock().await;
                let index_result = chat_ids.iter().position(|&x| x == chat_id);
                if index_result.is_some() {
                    chat_ids.remove(index_result.unwrap());
                    persist::save_chat_ids_or_print_error(&chat_ids);
                    send_message(
                        &client,
                        &bot_token,
                        chat_id,
                        "OK, I won't greet you anymore. :'(",
                    )
                    .await;
                }
            } else {
                let mut chat_ids = chat_ids.lock().await;
                let index_result = chat_ids.iter().position(|&x| x == chat_id);
                if index_result.is_some() {
                    send_message(
                        &client,
                        &bot_token,
                        chat_id,
                        "You are already being greeted! Chill. :P",
                    )
                    .await;
                } else {
                    chat_ids.push(chat_id);
                    persist::save_chat_ids_or_print_error(&chat_ids);
                    send_message(
                        &client,
                        &bot_token,
                        chat_id,
                        "I'll greet you when it's morning. :)",
                    )
                    .await
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
}

async fn get_updates(
    client: &reqwest::Client,
    token: &str,
    last_update_id: i64,
) -> Result<TelegramUpdatesResponse, reqwest::Error> {
    let endpoint = format!("getUpdates?timeout=20&offset={}", last_update_id + 1);
    let req = client
        .get(get_telegram_api_url(token, &endpoint))
        .send()
        .await;
    if let Err(e) = req {
        Err(e)
    } else {
        req.unwrap().json::<TelegramUpdatesResponse>().await
    }
}

pub async fn send_message(
    client: &reqwest::Client,
    token: &str,
    chat_id: i64,
    message: &str,
) -> () {
    let mut body = HashMap::new();
    body.insert("chat_id", chat_id.to_string());
    body.insert("text", String::from(message));
    let url = get_telegram_api_url(token, "sendMessage");
    let response_result = client.post(url).json(&body).send().await;
    handle_maybe_request_failure(response_result, chat_id, "message");
}

pub async fn send_sticker(
    client: &reqwest::Client,
    token: &str,
    chat_id: i64,
    sticker_id: &str,
) -> () {
    let mut body = HashMap::new();
    body.insert("chat_id", chat_id.to_string());
    body.insert("sticker", String::from(sticker_id));
    let url = get_telegram_api_url(token, "sendSticker");
    let response_result = client.post(url).json(&body).send().await;
    handle_maybe_request_failure(response_result, chat_id, "sticker");
}

fn handle_maybe_request_failure(
    response_result: Result<Response, reqwest::Error>,
    chat_id: i64,
    request_type: &str,
) -> () {
    if let Err(err) = response_result {
        log::error!("Failed to send message for chat ID {}: {}", chat_id, err);
    } else {
        let response = response_result.unwrap();
        if response.status().is_client_error() || response.status().is_server_error() {
            log::error!(
                "Failed to send {} for chat ID {}, got an status code {}: {:?}",
                request_type,
                chat_id,
                response.status(),
                response
            );
        }
    }
}

fn get_telegram_api_url(token: &str, endpoint: &str) -> String {
    format!("https://api.telegram.org/bot{}/{}", token, endpoint)
}
