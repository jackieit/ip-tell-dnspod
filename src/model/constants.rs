use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

pub static USERNAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9\-\_@]{4,}$").unwrap());

pub static JWT_SECRET: &str = "EDs-ARpmZLI_eSX-LOMzt6B6abs07dmgj4sSe7woO-4";
pub static AES_KEY: &str = "mBZV0TBF5kX8RbX2kxEX4g==";

#[derive(Debug, Serialize)]
pub struct RespMsg {
    pub code: Option<i32>,
    pub message: String,
}
