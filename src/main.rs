use std::{
    process,
    sync::{Arc, Mutex},
};
use timely_greeter_bot::{env, persist, telegram::*, timezone};

type ShareableIds = Arc<Mutex<Vec<i64>>>;

fn main() {
    pretty_env_logger::init_timed();
    let bot_token = env::load_bot_token().unwrap_or_else(|err| {
        log::error!("Could not read bot token: {}", err);
        process::exit(1);
    });

    log::info!("Starting bot...");

    let chat_ids: ShareableIds = Arc::new(Mutex::new(persist::get_stored_chat_ids().unwrap()));
    spawn_bot_loop(&chat_ids, &bot_token);
    timezone::greeter_loop::run_greeter_loop_in_current_thread(&chat_ids, &bot_token);
}
