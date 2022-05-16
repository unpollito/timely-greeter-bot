use std::sync::{Arc, Mutex};

extern crate chrono;
extern crate chrono_tz;

pub mod env;
pub mod error;
pub mod persist;
pub mod telegram;
pub mod timezone;

pub type ShareableIds = Arc<Mutex<Vec<i64>>>;
