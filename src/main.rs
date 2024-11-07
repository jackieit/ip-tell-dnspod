use crate::error::ItdResult;
use crate::ipaddr::{ipv6_net::Ipv6Net, IpAddrExt, IpType};
use crate::model::records::Records;
use sqlx::sqlite::SqlitePool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod dnspod;
mod error;
mod ipaddr;
mod model;
mod utils;

#[derive(Debug, Clone)]
pub struct IpState {
    ipv4: Option<String>,
    ipv4_updated_at: i64,
    ipv6: Option<String>,
    ipv6_updated_at: i64,
}
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let ipaddr = Ipv6Net::new("test-ipv6.com".to_string(), IpType::V6);
    let ip_state = Arc::new(Mutex::new(IpState {
        ipv4: None,
        ipv4_updated_at: 0,
        ipv6: None,
        ipv6_updated_at: 0,
    }));
    let db = SqlitePool::connect("sqlite:dnspod.db").await;
    if db.is_err() {
        println!("db connect failed");
        return;
    }
    let db = db.unwrap().clone();
    let record_model = Records::new(&db);
    let handle = tokio::spawn(async move {
        loop {
            // Do some work here
            println!("Thread is working...");
            let ip = ipaddr.get_ip(ip_state.clone()).await;
            match ip {
                Ok(ip_changed) => {
                    if ip_changed {
                        record_model.update_by_ip(ip_state.clone()).await;
                    } else {
                        println!("IP not changed!");
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
            // Sleep for 1 second between iterations
            thread::sleep(Duration::from_secs(10));
        }
    });
    handle.await.unwrap();
    //let ipadd =
    //println!("ip: {:?}", ip_state.lock().unwrap().ipv6);
    // match ip {
    //     Ok(_) => println!("ip: {:?}", ip_state.lock().unwrap()),
    //     Err(e) => println!("err: {}", e),
    // }
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
