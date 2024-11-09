use axum::{
    // http::{ Request},
    extract::Request,
    middleware::Next,
    response::Response,
};
pub async fn propagate_header(request: Request, next: Next) -> Response {
    // do something with `request`...

    let mut response = next.run(request).await;
    //response.headers_mut().insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    response
        .headers_mut()
        .insert("Server", "GS/1.0".parse().unwrap());
    response
        .headers_mut()
        .insert("X-Powered-By", "You-Guess".parse().unwrap());
    response
}
