use crate::dnspod::action::PodAction;
use crate::error::ItdResult;
use crate::ipaddr::IpAddrExt;
use crate::ipaddr::IpType;
use crate::model::records::Records;
use crate::IpState;
use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::info;
//use tokio::task;
use crate::ipaddr::ipv6_net::Ipv6Net;
use tokio::sync::RwLock;

pub async fn task(db: SqlitePool, ip_state: Arc<RwLock<IpState>>) -> ItdResult<()> {
    // Do some work here
    info!("Thread is working...");
    let ipaddr = Ipv6Net::new();
    //task::spawn(async move {
    let (ip_v4_changed, ip_v6_changed) = ipaddr
        .get_ip(vec![IpType::V4, IpType::V6], ip_state.clone())
        .await?;
    if ip_v4_changed || ip_v6_changed {
        info!("IP changed!");
        let record_model = Records::new(&db);
        let lists = record_model.get_record_list().await?;
        let ip_state = ip_state.read().await;
        let ipv4 = ip_state.ipv4.clone();
        let ipv6 = ip_state.ipv6.clone();
        drop(ip_state); 
        for item in lists {
            let ip_value = match item.ip_type.as_str() {
                "A" => {
                    if !ip_v4_changed {
                        continue;
                    }
                    ipv4.clone().unwrap()
                }
                "AAAA" => {
                    if !ip_v6_changed {
                        continue;
                    }
                    ipv6.clone().unwrap()
                }
                _ => {
                    info!("Invalid ip type! {}", item.ip_type);
                    continue;
                }
            };
            let domain = format!("{}.{}", item.host, item.domain);
            info!("Update record domain: {} ,ip: {}", domain, &ip_value);
            //let query_db = Arc::clone(&db);
            let _ = sqlx::query!(
                r#"UPDATE user_domain SET ip = ? WHERE id = ?"#,
                ip_value,
                item.id
            )
            .execute(&db)
            .await?;
            let action = PodAction::new(&db, item.appid).await?;
            let _ = action
                .modify_record(
                    &item.host,
                    &item.domain,
                    item.record_id,
                    &item.ip_type,
                    &ip_value,
                    600,
                )
                .await?;
        }
    }
    Ok(())
}
