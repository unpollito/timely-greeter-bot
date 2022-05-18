use std::{
    process,
    sync::{Arc, Mutex},
};
use timely_greeter_bot::{env, persist, telegram::*, timezone};

type ShareableIds = Arc<Mutex<Vec<i64>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let bot_token = env::load_bot_token().unwrap_or_else(|err| {
        log::error!("Could not read bot token: {}", err);
        process::exit(1);
    });

    log::info!("Starting bot...");

    let chat_ids: ShareableIds = Arc::new(Mutex::new(persist::get_stored_chat_ids().unwrap()));

    let bot_loop_handle = run_bot_loop(&chat_ids, &bot_token);
    let greeter_handle = timezone::greeter_loop::run_greeter_loop(&chat_ids, &bot_token);

    let (first, second) = tokio::join!(bot_loop_handle, greeter_handle);
    if let Err(err) = first {
        log::error!("Error in bot loop: {}", err);
    }
    if let Err(err) = second {
        log::error!("Error in greeter loop: {}", err);
    }
}
