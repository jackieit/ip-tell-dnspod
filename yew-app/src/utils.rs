use gloo_net::http::Request;
use web_sys::window;

use crate::error::{AppResult, Error};

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}
impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}
#[derive(Debug, Clone, Default)]
pub struct RequestOptions<T>
where
    T: serde::Serialize,
{
    pub method: HttpMethod,
    pub uri: String,
    pub body: Option<T>,
}
pub async fn request<T, R>(options: RequestOptions<T>) -> AppResult<R>
where
    T: serde::Serialize,
    R: serde::de::DeserializeOwned,
{
    let mut request_builder = match options.method {
        HttpMethod::GET => Request::get(&options.uri),
        HttpMethod::POST => Request::post(&options.uri),
        HttpMethod::PUT => Request::put(&options.uri),
        HttpMethod::DELETE => Request::delete(&options.uri),
        // _ => return Err(Error::InvalidMethod),
    };
    request_builder = request_builder.header("Content-Type", "application/json");
    //todo add token
    let local_storage = window().unwrap().local_storage().unwrap().unwrap();
    let token = if let Some(name) = local_storage.get_item("itd_token").unwrap() {
        name
    } else {
        "".to_string()
    };
    request_builder = request_builder.header("Authorization", &format!("Bearer {}", token));
    
    let response = if let Some(body) = options.body {
        request_builder.json(&body)?.send().await?
    } else {
        request_builder.send().await?
    };

    let status = response.status();
    if status != 200 {
        let response_body: serde_json::Value = response.json().await.unwrap();
        return Err(Error::new(
            format!("{}", response_body["code"]),
            format!("{}", response_body["message"]),
        ));
    } else {
        let response_body: R = response.json().await.unwrap();
        return Ok(response_body);
    }
}
