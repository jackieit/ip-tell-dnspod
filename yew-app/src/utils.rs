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
pub struct RequestOptions
{
    pub method: HttpMethod,
    pub uri: String,
    pub body: Option<String>,
}
pub async fn request< R>(options: RequestOptions) -> AppResult<R>
where
    R: serde::de::DeserializeOwned,
{
    let uri: String = if options.uri.starts_with("http") {
        options.uri
    } else {
        format!("http://localhost:3310/v1{}",options.uri)
    };
    
    let mut request_builder = match options.method {
        HttpMethod::GET => Request::get(&uri),
        HttpMethod::POST => Request::post(&uri),
        HttpMethod::PUT => Request::put(&uri),
        HttpMethod::DELETE => Request::delete(&uri),
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
        let body:serde_json::Value = serde_json::from_str(&body).map_err(|_| Error::new("Invalid JSON".to_string(), "Invalid JSON".to_string()))?;

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
