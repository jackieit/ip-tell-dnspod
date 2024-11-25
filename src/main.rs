use crate::dnspod::action::PodAction;
use crate::error::ItdResult;
use crate::ipaddr::{ipv6_net::Ipv6Net, IpAddrExt, IpType};
use crate::model::records::Records;
use crate::utils::log_setup;
use crate::web::main::http_server;

//use model::app;
use tracing::{error, info};

use sqlx::sqlite::SqlitePool;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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
    pub ip_state: Arc<Mutex<IpState>>,
}
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = ItdResult<T>> + Send + 'a>>;

#[tokio::main]
async fn main() {
    log_setup();
    info!("Welcome to Ip Tell DnsPod!");
    let ipaddr = Ipv6Net::new("test-ipv6.com".to_string(), IpType::V4);
    let app_state = get_app_state().await;
    let ip_state = app_state.ip_state.clone();
    let db = app_state.db.clone();
    tokio::spawn(async move {
        loop {
            // Do some work here
            info!("Thread is working...");
            let ip = ipaddr.get_ip(ip_state.clone()).await;
            match ip {
                Ok(ip_changed) => {
                    if ip_changed {
                        let record_model = Records::new(&db);
                        let lists = record_model.get_record_list().await;
                        //println!("Record list: {:?}", lists);
                        match lists {
                            Err(e) => {
                                error!("Error: {}", e);
                            }
                            Ok(lists) => {
                                if lists.len() == 0 {
                                    info!("No record found!");
                                    continue;
                                }
                                for item in lists {
                                    let ip_value = match item.ip_type.as_str() {
                                        "A" => ip_state.lock().unwrap().ipv4.clone().unwrap(),
                                        "AAAA" => ip_state.lock().unwrap().ipv6.clone().unwrap(),
                                        _ => {
                                            info!("Invalid ip type! {}", item.ip_type);
                                            continue;
                                        }
                                    };
                                    let domain = format!("{}.{}", item.host, item.domain);
                                    info!("Update record domain: {} ,ip: {}", domain, &ip_value);
                                    //let query_db = Arc::clone(&db);
                                    let result = sqlx::query!(
                                        r#"UPDATE user_domain SET ip = ? WHERE id = ?"#,
                                        ip_value,
                                        item.id
                                    )
                                    .execute(&db)
                                    .await;
                                    match result {
                                        Err(e) => {
                                            error!("Update {} Error: {}", domain, e);
                                        }
                                        Ok(_) => {
                                            let action = PodAction::new(&db, item.appid).await;
                                            if action.is_err() {
                                                error!(
                                                    "Update {} Error: {}",
                                                    domain,
                                                    action.err().unwrap()
                                                );
                                                continue;
                                            }
                                            let action = action.unwrap();

                                            let result = action
                                                .modify_record(
                                                    &item.host,
                                                    &item.domain,
                                                    item.record_id,
                                                    &item.ip_type,
                                                    &ip_value,
                                                    600,
                                                )
                                                .await;
                                            if result.is_err() {
                                                error!(
                                                    "Update {} Error: {}",
                                                    domain,
                                                    result.err().unwrap()
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        info!("IP not changed!");
                    }
                }
                Err(e) => error!("Error: {}", e),
            }
            // Sleep for 1 second between iterations
            thread::sleep(Duration::from_secs(10));
        }
    });
    //handle.await.unwrap();
    http_server(app_state.clone()).await;
}
pub async fn get_app_state() -> Arc<AppState> {
    let ip_state = Arc::new(Mutex::new(IpState {
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
