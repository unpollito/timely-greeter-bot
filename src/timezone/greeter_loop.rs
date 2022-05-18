use std::error::Error;

use tokio::time::{sleep, Duration};

use crate::ShareableIds;

const GOOD_MORNING_STICKER_ID: &str =
    "CAACAgIAAxkBAAMHYoAnQ-mjFlYcQI7MY6ofspGVa50AAjkBAAIQIQIQ0zO07gSDOlQkBA";

pub async fn run_greeter_loop(
    chat_ids: &ShareableIds,
    bot_token: &str,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut greeter = super::TimezoneGreeter::new();
    loop {
        sleep(Duration::from_secs(60)).await;
        let cities_to_greet = greeter.get_cities_to_greet();
        if let Some(message) = super::get_good_morning_message(&cities_to_greet) {
            log::info!("{}", message);
            let chat_ids = lock_and_clone_chat_ids(chat_ids);
            for chat_id in chat_ids {
                crate::telegram::send_sticker(
                    &client,
                    &bot_token,
                    chat_id,
                    GOOD_MORNING_STICKER_ID,
                )
                .await;
                crate::telegram::send_message(&client, &bot_token, chat_id, &message).await;
            }
        } else {
            log::debug!("No cities to greet");
        }
    }
}

fn lock_and_clone_chat_ids(chat_ids: &ShareableIds) -> Vec<i64> {
    let chat_ids = chat_ids.lock().unwrap();
    chat_ids.clone()
}
