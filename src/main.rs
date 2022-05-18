use std::{process, sync::Arc};
use timely_greeter_bot::{env, persist, telegram::*, timezone, ShareableIds};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
    let bot_token = env::load_bot_token().unwrap_or_else(|err| {
        log::error!("Could not read bot token: {}", err);
        process::exit(1);
    });

    log::info!("Starting bot...");

    let chat_ids: ShareableIds = Arc::new(Mutex::new(persist::get_stored_chat_ids().unwrap()));
    let bot = SimpleTelegramBot::new(bot_token);
    let sender = bot.sender();

    let bot_loop_handle = run_bot_loop(chat_ids.clone(), sender.clone(), bot.updater());
    let greeter_handle = timezone::greeter_loop::run_greeter_loop(&chat_ids, sender.clone());

    let (first, second) = tokio::join!(bot_loop_handle, greeter_handle);
    if let Err(err) = first {
        log::error!("Error in bot loop: {}", err);
    }
    if let Err(err) = second {
        log::error!("Error in greeter loop: {}", err);
    }
}
