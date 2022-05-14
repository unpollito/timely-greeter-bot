use chrono::{self, TimeZone};
use chrono_tz;
use std::process;
use std::{thread, time};
use timely_greeter_bot::env;
use timely_greeter_bot::timezone;

// 1. Get timezone info
// t1 Read messages
// t2 Send messages
// 2. Clean up threads

fn main() {
    let bot_token = env::load_bot_token().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    println!("Bot token: {}", bot_token);

    let mut greeter = timezone::TimezoneGreeter::new();

    loop {
        let cities = greeter.get_cities_to_greet();
        let now_utc = chrono::Utc::now().naive_utc();
        let now = chrono_tz::Europe::Berlin.from_utc_datetime(&now_utc);
        if cities.len() > 0 {
            println!(
                "{}: Good morning to {}",
                now.to_rfc3339(),
                cities.join(", ")
            );
        } else {
            println!("{}: nothing to greet", now.to_rfc3339());
        }
        thread::sleep(time::Duration::from_millis(60000));
    }
}
