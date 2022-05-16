use std::{thread, time::Duration};

use crate::ShareableIds;

const GOOD_MORNING_STICKER_ID: &str =
    "CAACAgIAAxkBAAMHYoAnQ-mjFlYcQI7MY6ofspGVa50AAjkBAAIQIQIQ0zO07gSDOlQkBA";

pub fn run_greeter_loop_in_current_thread(chat_ids: &ShareableIds, bot_token: &str) -> ! {
    let client = reqwest::blocking::Client::new();
    let mut greeter = super::TimezoneGreeter::new();
    loop {
        thread::sleep(Duration::from_secs(60));
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
                );
                crate::telegram::send_message(&client, &bot_token, chat_id, &message);
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
