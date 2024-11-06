use crate::error::ItdResult;
use crate::ipaddr::{ipv6_net::Ipv6Net, IpAddrExt, IpType};
use sqlx::sqlite::SqlitePool;
mod dnspod;
mod error;
mod ipaddr;
mod model;
mod utils;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut ipaddr = Ipv6Net::new("test-ipv6.com".to_string(), IpType::V4);
    let ip = ipaddr.do_request().await;
    match ip {
        Ok(ip) => println!("ip: {}", ip),
        Err(e) => println!("err: {}", e),
    }
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
