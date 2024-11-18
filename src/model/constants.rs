use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

pub static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-Z0-9\-\_@]{4}$").unwrap());
#[derive(Debug, Serialize)]
pub struct RespMsg {
    pub code: Option<i32>,
    pub message: String,
}
