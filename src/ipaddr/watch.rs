use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::info;
use crate::error::ItdResult;
use crate::ipaddr::IpAddrExt;
use crate::ipaddr::IpType;
use crate::IpState;
use crate::model::records::Records;
use crate::dnspod::action::PodAction;
use tokio::task;
use tokio::sync::RwLock;
use crate::ipaddr::ipv6_net::Ipv6Net;

pub fn task(db: SqlitePool, ip_state: Arc<RwLock<IpState>>) -> task::JoinHandle<ItdResult<()>>{
   // Do some work here
   info!("Thread is working...");
   let ipaddr = Ipv6Net::new();
   task::spawn(async move {
        let (ip_v4_changed, ip_v6_changed) = ipaddr.get_ip(vec![IpType::V4,IpType::V6],ip_state.clone()).await?;
        if  ip_v4_changed || ip_v6_changed {
            info!("IP changed!");
            let record_model = Records::new(&db);
            let lists = record_model.get_record_list().await?;
            
            for item in lists {
              
                let ip_value = match item.ip_type.as_str() {
                    "A"  => {
                        if !ip_v4_changed {
                            continue;
                        }
                        ip_state.read().await.ipv4.clone().unwrap()
                    },
                    "AAAA" => {
                        if !ip_v6_changed {
                            continue;
                        }
                        ip_state.read().await.ipv6.clone().unwrap()
                    },
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
                let _ = action.modify_record(&item.host,
                                                &item.domain,
                                                item.record_id,
                                                &item.ip_type,
                                                &ip_value,
                                                600,
                                            )
                                            .await?;
                
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Ok(())
   })
 
   // Sleep for 1 second between iterations
  
}
