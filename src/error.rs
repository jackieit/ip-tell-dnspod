use std::fmt;
use std::io::Error as IoError;
use std::string::FromUtf8Error as Utf8Error;
use std::time::SystemTimeError as TimeError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;

#[derive(Debug)]
pub struct ItdError(String, String);

impl ItdError {
  pub fn new(kind: String, message: String) -> ItdError {
    ItdError(kind, message)
  }
}
impl fmt::Display for ItdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "IpTellDnspod Error: From {}, {}", self.0, self.1)
  }
}
impl std::error::Error for ItdError {}
impl From<IoError> for ItdError {
    fn from(err: IoError) -> Self {
      ItdError::new("StdIo".to_string(), err.to_string())
    }
}
impl From<Utf8Error> for ItdError {
    fn from(err: Utf8Error) -> Self {
      ItdError::new("FromUtf8".to_string(), err.to_string())
    }
}
impl From<TimeError> for ItdError {
    fn from(err: TimeError) -> Self {
      ItdError::new("SystemTime".to_string(), err.to_string())
    }
}
impl From<JsonError> for ItdError {
    fn from(err: JsonError) -> Self {
      ItdError::new("JsonConvert".to_string(), err.to_string())
    }
}
impl From<ReqwestError> for ItdError {
  fn from(err: ReqwestError) -> Self {
    ItdError::new("Reqwest".to_string(), err.to_string())
  }
}
pub type ItdResult<T> = Result<T, ItdError>;
