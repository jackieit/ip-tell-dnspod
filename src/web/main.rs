//use axum_server::tls_rustls::RustlsConfig;
use http::{header, header::HeaderName, Method};
use std::net::SocketAddr;
use std::sync::Arc;
//use std::time::Duration;
use tokio::signal;
use tokio::task::JoinHandle;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    services::{ServeDir, ServeFile},
    trace,
};
use axum::{extract::connect_info::MockConnectInfo, Router};
use tracing::info;

use crate::{
    web::middleware::{auth::auth, header::propagate_header, log_bearer::make_span_with},
    AppState,
};



pub async fn create_app(app_state: Arc<AppState>) -> Router {
    let root = format!("{}/wwwroot", env!("CARGO_MANIFEST_DIR"));
    let serve_service = ServeDir::new(root.clone())
        .append_index_html_on_directories(true)
        .not_found_service(ServeFile::new(root.clone() + "404.html"));
    // h5 create react app h5 page
    let create_react_app = ServeDir::new(root + "/h5")
        .append_index_html_on_directories(true)
        .not_found_service(ServeFile::new("wwwroot/h5/index.html"));
    Router::new()
        .merge(super::routes::app::create_route())
        .merge(super::routes::record::create_route())
        .merge(super::routes::user::create_route())
        .nest_service("/h5", create_react_app)
        .nest_service("/", serve_service)
        .with_state(app_state.clone())
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn_with_state(app_state, auth))
                .layer(axum::middleware::from_fn(propagate_header))
                .layer(
                    trace::TraceLayer::new_for_http()
                        //.make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                        .make_span_with(make_span_with)
                        .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
                )
                .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                    header::AUTHORIZATION,
                )))
                // Compress responses
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::DELETE,
                            Method::OPTIONS,
                        ])
                        //.allow_origin(app_env.allow_origins)
                        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                        .allow_credentials(true),
                    //.expose_headers([header::HeaderName::from_static("x-request-id")]),
                )
                // Propagate `X-Request-Id`s from requests to responses
                .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                    "x-request-id",
                ))),
        )
}
pub async fn http_server(app_state: Arc<AppState>, handle: JoinHandle<()>) {
    println!("listening on {:?}", &app_state);
    let address = SocketAddr::from(([0, 0, 0, 0], 3310));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    let app = create_app(app_state).await;

    info!("listening on {}", &address);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(handle))
    .await
    .expect("Failed to start server");
}
async fn shutdown_signal(handle: JoinHandle<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    handle.abort();
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
// only used for test
#[allow(dead_code)]
pub async fn test_app() -> Router {
    let app_state = crate::get_app_state().await;
    let mut app = create_app(app_state).await;
    app = app.layer(MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 3002))));
    app
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::error::ItdResult;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use serde_json::Value;
    use tower::ServiceExt;
    /// 测试请求
    /// #Arguments
    /// * `uri` - 请求的uri
    /// * `body` - 请求的body
    /// * `method` - 请求的方法
    /// * `token` - 请求的token
    /// #Returns
    /// * `ItdResult<Value>` - 返回的结果
    pub async fn request(
        url: &str,
        method: &str,
        body: Option<&str>,
        token: Option<&str>,
    ) -> ItdResult<Value> {
        let app = test_app().await;
        let method = method.parse::<http::Method>().unwrap();
        let request = Request::builder()
            .method(method)
            .uri(url)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(http::header::USER_AGENT, "tokio-test");
        let request = match token {
            Some(token) => {
                request.header(http::header::AUTHORIZATION, "Bearer ".to_string() + token)
            }
            None => request,
        };
        let body = match body {
            Some(body) => {
                let body_json = serde_json::from_str::<serde_json::Value>(body).unwrap();
                Body::from(serde_json::to_vec(&body_json).unwrap())
            }
            None => Body::empty(),
        };

        let response = app.oneshot(request.body(body).unwrap()).await.unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        Ok(body)
    }
}
