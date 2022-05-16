use std::error::Error;
use std::fs;
use std::io::ErrorKind;

const CHAT_IDS_FILE: &str = "chat_ids.txt";
const UPDATE_ID_FILE: &str = "update_id.txt";

pub fn get_stored_chat_ids() -> Result<Vec<i64>, Box<dyn Error>> {
    match fs::read_to_string(CHAT_IDS_FILE) {
        Ok(content) => {
            let mut result: Vec<i64> = Vec::new();
            for line in content.lines() {
                let parse_result = line.parse::<i64>();
                if parse_result.is_err() {
                    return Ok(Vec::new());
                }
                result.push(parse_result.unwrap());
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

pub fn get_stored_update_id() -> Result<i64, Box<dyn Error>> {
    match fs::read_to_string(UPDATE_ID_FILE) {
        Ok(content) => {
            let parse_result = content.parse::<i64>();
            if parse_result.is_err() {
                return Ok(0);
            }
            return Ok(parse_result.unwrap());
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                return Ok(0);
            }
            Err(Box::new(err))
        }
    }
}

pub fn save_chat_ids(ids: &[i64]) -> Result<(), Box<dyn Error>> {
    let content = ids
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join("\r\n");
    match fs::write(CHAT_IDS_FILE, content) {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}

pub fn save_update_id(id: i64) -> Result<(), Box<dyn Error>> {
    match fs::write(UPDATE_ID_FILE, id.to_string()) {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}
