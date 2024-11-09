use std::collections::HashMap;

use axum::http::Request;

use tracing::Span;
pub fn make_span_with<B>(req: &Request<B>) -> Span {
    let method = req.method().clone();
    let uri = req.uri().clone();

    // Filter headers and remove the "Authorization" header
    let filtered_headers: HashMap<_, _> = req
        .headers()
        .iter()
        //.filter(|(key, _)| key != &"authorization")
        .map(|(key, value)| {
            if key != &"authorization" {
                (key.to_string(), value.to_str().unwrap_or("").to_string())
            } else {
                (key.to_string(), "Bearer *****".to_string())
            }
        })
        .collect();

    // Create a span with filtered headers
    tracing::info_span!(
        "request",
        method = %method,
        uri = %uri,
        headers = ?filtered_headers,
    )
}
