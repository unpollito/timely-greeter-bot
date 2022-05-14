use super::error::AppError;
use std::{fs, io};

pub fn load_bot_token() -> Result<String, AppError> {
    let env_file = fs::read_to_string(".env");

    match env_file {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Err(AppError {
                message: String::from(".env file not found"),
            }),
            _ => {
                let mut message = String::from("I/O error: {}");
                message.push_str(&e.to_string());
                Err(AppError { message })
            }
        },
        Ok(content) => {
            for line in content.lines() {
                if line.starts_with("TELEGRAM_BOT_TOKEN=") {
                    return Ok(String::from(&line[19..]));
                }
            }
            Err(AppError {
                message: String::from("TELEGRAM_BOT_TOKEN not found"),
            })
        }
    }
}
