use super::{persist, ShareableIds};
use simple_telegram_bot::{SimpleTelegramBotSender, SimpleTelegramBotUpdater};
use std::error::Error;

pub async fn run_bot_loop(
    chat_ids: ShareableIds,
    sender: SimpleTelegramBotSender,
    updater: SimpleTelegramBotUpdater,
) -> Result<(), Box<dyn Error>> {
    let chat_ids = chat_ids.clone();
    let mut stream = updater.updates();

    loop {
        match stream.recv().await {
            None => break,
            Some(response) => {
                for update in response.result {
                    if let Some(message) = update.message {
                        let text = match message.text {
                            Some(text) => text,
                            None => String::from("[no message]"),
                        };
                        let chat_id = message.chat.id;
                        log::info!(
                            "Received message from {} (chat ID: {}): \"{}\"",
                            message.from.username,
                            chat_id,
                            text
                        );

                        if text == "/stop" {
                            let mut chat_ids = chat_ids.lock().await;
                            let index_result = chat_ids.iter().position(|&x| x == chat_id);
                            if index_result.is_some() {
                                chat_ids.remove(index_result.unwrap());
                                persist::save_chat_ids_or_print_error(&chat_ids);
                                sender
                                    .send_message(chat_id, "OK, I won't greet you anymore. :'(")
                                    .await;
                            }
                        } else {
                            let mut chat_ids = chat_ids.lock().await;
                            let index_result = chat_ids.iter().position(|&x| x == chat_id);
                            if index_result.is_some() {
                                sender
                                    .send_message(
                                        chat_id,
                                        "You are already being greeted! Chill. :P",
                                    )
                                    .await;
                            } else {
                                chat_ids.push(chat_id);
                                persist::save_chat_ids_or_print_error(&chat_ids);
                                sender
                                    .send_message(chat_id, "I'll greet you when it's morning. :)")
                                    .await
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
