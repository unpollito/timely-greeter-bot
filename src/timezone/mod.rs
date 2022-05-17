use chrono::{self, Datelike, TimeZone, Timelike};
use chrono_tz;
use std::collections::HashMap;

mod constants;

pub mod greeter_loop;

pub struct TimezoneGreeter {
    last_greeted: HashMap<String, chrono::DateTime<chrono_tz::Tz>>,
}

impl TimezoneGreeter {
    pub fn new() -> TimezoneGreeter {
        TimezoneGreeter {
            last_greeted: HashMap::new(),
        }
    }

    // TODO:
    // This could be a hell of a lot more efficient.
    // For instance, if I just ran this loop once and stored the next UTC
    // timestamp when I should greet each place, then just looped over the list
    // of timestamps to see which ones are due.
    // But at the end of the day, this just takes ~0.01s to run, it's done
    // once per minute (at the time of writing) and I don't think it's a good
    // time investment.
    pub fn get_cities_to_greet(&mut self) -> Vec<String> {
        let mut cities_to_greet: Vec<String> = vec![];
        let now = chrono::Utc::now().naive_utc();
        for tz in constants::TIMEZONES {
            let tz_time = tz.from_utc_datetime(&now);
            let does_time_match =
                tz_time.hour() == constants::GREET_AT_HOUR && tz_time.minute() < 5;
            let stored_time = self.last_greeted.get(tz.name());
            let is_yet_to_greet_today =
                stored_time.is_none() || stored_time.unwrap().day() != tz_time.day();

            if does_time_match && is_yet_to_greet_today {
                cities_to_greet.push(get_timezone_name(tz));
                self.last_greeted.insert(tz.name().to_string(), tz_time);
            }
        }
        cities_to_greet
    }
}

fn get_timezone_name(tz: chrono_tz::Tz) -> String {
    match tz.name() {
        "Antarctica/Casey" => String::from("Casey Island"),
        "Atlantic/Canary" => String::from("Canary Islands"),
        "Atlantic/Faroe" => String::from("Faroe Islands"),
        "America/Curacao" => String::from("Curaçao"),
        "America/PortauPrince" => String::from("Port-au-prince"),
        "America/St_Johns" => String::from("St John's"),
        "Antarctica/DumontDUrville" => String::from("Dumont d'Urville"),
        "Australia/Lord_Howe" => String::from("Lord Howe Island"),
        "Indian/Christmas" => String::from("Christmas Island"),
        "Indian/Cocos" => String::from("Cocos Islands"),
        "Pacific/Chatham" => String::from("Chatham Islands"),
        "Pacific/Easter" => String::from("Easter Island"),
        "Pacific/Enderbury" => String::from("Enderbury Island"),
        "Pacific/Gambier" => String::from("Gambier Islands"),
        "Pacific/Marquesas" => String::from("Marquesas Islands"),
        other => {
            let full_name = String::from(other);
            let split: Vec<&str> = full_name.split("/").collect();
            String::from(split[split.len() - 1]).replace("_", " ")
        }
    }
}

pub fn get_good_morning_message(cities: &Vec<String>) -> Option<String> {
    if cities.len() == 0 {
        None
    } else if cities.len() == 1 {
        Some(format!("Good morning to {}!", cities[0]))
    } else {
        Some(format!(
            "Good morning to {}, and {}!",
            cities[..cities.len() - 1].join(", "),
            cities[cities.len() - 1],
        ))
    }
}
