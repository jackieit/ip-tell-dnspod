use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use crate::error::{ItdError, ItdResult};
use crate::{ AppState,err};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::Level;
//use tracing_subscriber::{fmt, layer::SubscriberExt};
use crate::model::constants::{AES_KEY, JWT_SECRET};
use tracing_subscriber::fmt::writer::MakeWriterExt;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
    //pub id: i32,
}

/// 获取当前时间戳
pub fn timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let timestamp = since_the_epoch.as_secs();
    timestamp as i64
}
/// hash user password
/// return hash string
pub fn password_hash(password: &str) -> ItdResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

/// verify password
/// return bool
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
pub fn validate_password(password: &str) -> Result<(), validator::ValidationError> {
    let length_and_characters = Regex::new(r"^[A-Za-z\d@$!%*?&#]{8,}$").unwrap();
    if !length_and_characters.is_match(password) {
        return Err(validator::ValidationError::new("密码码格式错误"));
    }
    // Check if the password contains at least one letter
    let has_letter = Regex::new(r"[a-zA-Z]").unwrap();
    if !has_letter.is_match(password) {
        return Err(validator::ValidationError::new("需要至少一个字符"));
    }

    // Check if the password contains at least one digit
    let has_digit = Regex::new(r"\d").unwrap();
    if !has_digit.is_match(password) {
        return Err(validator::ValidationError::new("需要至少一个数字"));
    }

    // Check if the password contains at least one special character
    let has_special_char = Regex::new(r"[@$!%*?&#]").unwrap();
    if !has_special_char.is_match(password) {
        return Err(validator::ValidationError::new("需要至少一个特殊字符"));
    }
    Ok(())
}
// encode accesstoken
pub fn encode_token(userid: i64, exp: i64) -> ItdResult<(String, i64)> {
    // let jwt_secret = "EDs-ARpmZLI_eSX-LOMzt6B6abs07dmgj4sSe7woO-4";
    let exp = timestamp() + exp;
    let claims = Claims {
        sub: userid,
        exp: exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )?;

    Ok((token, exp))
}
// decode accesstoken
pub fn decode_token(token: &str) -> ItdResult<Claims> {
    //let jwt_secret = "EDs-ARpmZLI_eSX-LOMzt6B6abs07dmgj4sSe7woO-4";
    let token_message = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_message.claims)
}

/// encrypt string with aes
pub fn encrypt_data(data: Vec<u8>) -> ItdResult<String> {
    let mch_key = STANDARD.decode(&AES_KEY)?;
    let mch_key = mch_key.as_slice();
    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
    let pt_len = data.len();
    let iv = [0u8; 16];
    let buf_len = pt_len + (16 - pt_len % 16);
    let mut buf = vec![0u8; buf_len];
    //let mut buf = buf.as_mut_slice();
    let data = &data[..];
    //let pt_len = data.len();
    buf[..pt_len].copy_from_slice(data);
    let cipher = Aes128CbcEnc::new_from_slices(mch_key, &iv).map_err(|_e| {
        ItdError::new(
            "encrypt_data".to_string(),
            "Aes128 loadkey error".to_string(),
        )
        //Error::DataError(DataValidationError::new("Aes128 loadkey erro".to_string()))
    })?;
    let ct = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .map_err(|_e| ItdError::new("encrypt_data".to_string(), "padding error".to_string()))?;
    Ok(STANDARD.encode(ct))
}

/// decrypt base64 data to Vec<u8>
pub fn decrypt_data(data: &str) -> ItdResult<Vec<u8>> {
    let mch_key = STANDARD.decode(&AES_KEY)?;

    let mch_key = mch_key.as_slice();
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
    let iv = [0u8; 16];

    let data = STANDARD.decode(data)?;
    let data = data.as_slice();
    let pt = Aes128CbcDec::new_from_slices(mch_key, &iv).unwrap();
    let buf_len = data.len() + (16 - data.len() % 16);
    let mut buf = vec![0u8; buf_len];

    let pt = pt
        .decrypt_padded_b2b_mut::<Pkcs7>(data, &mut buf)
        .map_err(|_e| ItdError::new("decrypt_data".to_string(), "unPading error".to_string()))?;
    Ok(pt.to_vec())
}
/// decrypt base64 data to string
pub fn decrypt_to_str(data: &str) -> ItdResult<String> {
    let data = decrypt_data(data)?;
    let data = String::from_utf8(data.into())?;
    Ok(data)
}

pub fn log_setup() {
    let level = Level::INFO;

    let logfile = tracing_appender::rolling::daily("logs", "main.log");
    let stdout = std::io::stdout.with_max_level(level);

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_file(true)
        .with_line_number(true)
        .with_writer(logfile.and(stdout))
        .init();
}
/// extract ip from app_state
pub async fn extract_ip(ip_type: &str, app_state: Arc<AppState>) -> ItdResult<String> {
    let ip_state = app_state.ip_state.read().await;
        let ip_value = match ip_type {
          "A" => {
            let ip = ip_state.ipv4.clone();
            if ip.is_some() {
              ip.unwrap()
            } else {
              return err!("No ipv4 address found");
            }
          },
          "AAAA" => {
            let ip = ip_state.ipv6.clone();
            if ip.is_some() {
              ip.unwrap()
            } else {
              return err!("No ipv6 address found");
            }
          },
          _ => {
              return err!("Invalid ip type");
           }
        };
        Ok(ip_value)
}
#[macro_export]
macro_rules! add_conn {
    ($struct_name:ident) => {
        pub struct $struct_name<'db> {
            db: &'db sqlx::Pool<sqlx::Sqlite>,
        }

        impl<'db> $struct_name<'db> {
            pub fn new(db: &'db sqlx::Pool<sqlx::Sqlite>) -> Self {
                $struct_name { db }
            }
        }
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    #[test]
    fn it_password_hash_works() {
        let password = "admin";
        let password = password_hash(password).unwrap();
        println!("{}", password)
    }
    #[test]
    fn generate_aes_key() {
       // assert!(matches!(key_size, 16 | 24 | 32), "Invalid AES key size");
        let mut key = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut key[..]);
        let result = STANDARD.encode(key);
        println!("{}", result);
    }
}
