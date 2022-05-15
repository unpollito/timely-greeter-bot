use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use telegram_bot::ChatId;

pub fn get_stored_chat_ids() -> Result<Vec<ChatId>, Box<dyn Error>> {
    match fs::read_to_string("chat_ids.txt") {
        Ok(content) => {
            let mut result: Vec<ChatId> = Vec::new();
            for line in content.lines() {
                let parse_result = line.parse::<i64>();
                if parse_result.is_err() {
                    return Ok(Vec::new());
                }
                result.push(ChatId::new(parse_result.unwrap()));
            }
            Ok(result)
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Ok(Vec::new());
            }
            Err(Box::new(err))
        }
    }
}

pub fn save_chat_ids(ids: &[ChatId]) -> Result<(), Box<dyn Error>> {
    let content = ids
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join("\r\n");
    match fs::write("chat_ids.txt", content) {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}
