use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};
use sqlx::sqlite::SqlitePool;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio::sync::broadcast::channel;

use crate::error::ItdResult;
use crate::utils::log_setup;
use crate::web::main::http_server;
use crate::ipaddr::watch::task;

mod dnspod;
mod error;
mod ipaddr;
mod model;
mod utils;
mod web;
#[derive(Debug, Clone)]
pub struct IpState {
    ipv4: Option<String>,
    ipv4_updated_at: i64,
    ipv6: Option<String>,
    ipv6_updated_at: i64,
}
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub ip_state: Arc<RwLock<IpState>>,
}
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = ItdResult<T>> + Send + 'a>>;

#[tokio::main]
async fn main() {
    log_setup();
    info!("Welcome to Ip Tell DnsPod!");
    
    let app_state = get_app_state().await;
    let ip_state = app_state.ip_state.clone();
    let db = app_state.db.clone();
    let (shutdown_tx, shutdown_rx) = channel::<()>(1);
    
    let handle = tokio::spawn({
        let mut rx = shutdown_tx.subscribe();
        async move {
            loop {
                tokio::select! {
                    _ = rx.recv() => {
                        info!("Background task received shutdown signal");
                        break;
                    }
                    _ = async {
                        let handle = task(db.clone(), ip_state.clone());
                        if let Err(err) = handle.await {
                            error!("Task failed: {}", err);
                        }
                        sleep(Duration::from_secs(60)).await;
                    } => {}
                }
            }
        }
    });
    // 设置 ctrl-c 处理
    tokio::spawn(async move {
        if let Ok(_) = tokio::signal::ctrl_c().await {
            info!("Received Ctrl+C, initiating shutdown");
            let _ = shutdown_tx.send(());
        }
    });
    http_server(app_state.clone(),handle, shutdown_rx).await;
    
}
pub async fn get_app_state() -> Arc<AppState> {
    let ip_state = Arc::new(RwLock::new(IpState {
        ipv4: None,
        ipv4_updated_at: 0,
        ipv6: None,
        ipv6_updated_at: 0,
    }));
    let db = get_conn().await;
    if db.is_err() {
        info!("db connect failed");
        return Arc::new(AppState {
            db: get_conn().await.unwrap(),
            ip_state,
        });
    }
    let db = db.unwrap();

    let share_state = Arc::new(AppState {
        db: db.clone(),
        ip_state: ip_state.clone(),
    });
    share_state
}
pub async fn get_conn() -> ItdResult<SqlitePool> {
    let pool = SqlitePool::connect("sqlite:dnspod.db").await?;
    return Ok(pool);
}
#[cfg(test)]
pub mod tests {
    use dotenv::dotenv;
    use std::collections::HashMap;
    use std::env;
    /**
     * 获取临时密钥
     */
    pub fn get_config() -> (String, String) {
        dotenv().ok();
        let env_map = env::vars()
            .into_iter()
            .map(|i| (i.0, i.1))
            .collect::<HashMap<String, String>>();
        let api_id = env_map.get("secret_id").unwrap().to_string();
        let secret_key = env_map.get("secret_key").unwrap().to_string();
        return (api_id, secret_key);
    }
}
