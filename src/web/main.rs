//use axum_server::tls_rustls::RustlsConfig;
use http::{header, header::HeaderName, Method};
use std::net::SocketAddr;
use std::sync::Arc;
//use std::time::Duration;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    services::{ServeDir, ServeFile},
    trace,
};

use crate::{
    web::middleware::{auth::auth, header::propagate_header, log_bearer::make_span_with},
    AppState,
};

use axum::{extract::connect_info::MockConnectInfo, Router};

use tracing::info;

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
        //.merge(routes::admin::create_route())
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
pub async fn http_server(app_state: Arc<AppState>) {
    println!("listening on {:?}", &app_state);
    let address = SocketAddr::from(([0, 0, 0, 0], 3310));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    let app = create_app(app_state).await;

    info!("listening on {}", &address);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Failed to start server");
}
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

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
pub async fn test_app(app_state: Arc<AppState>) -> Router {
    let mut app = create_app(app_state).await;
    app = app.layer(MockConnectInfo(SocketAddr::from(([0, 0, 0, 0], 3002))));
    app
}