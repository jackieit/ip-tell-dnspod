use std::fmt;


#[derive(Debug)]
pub struct Error(String, String);
impl Error {
  pub fn new(kind: String, message: String) -> Error {
    Error(kind, message)
  }
}
impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Error: From {}, {}", self.0, self.1)
  }
}
impl std::error::Error for Error {}

impl From<gloo_net::Error> for Error {
  fn from(err: gloo_net::Error) -> Self {
      Error::new("RequestError".to_string(), err.to_string())
  }
}

pub type AppResult<T> = std::result::Result<T, Error>;
