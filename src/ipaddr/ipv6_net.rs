///! 请求网站 test-ipv6.com
///！ 获取ipv4 https://ipv4.lookup.test-ipv6.com/ip/?asn=1&testdomain=test-ipv6.com&testname=test_asn4
///！ 获取ipv6 https://ipv6.lookup.test-ipv6.com/ip/?asn=1&testdomain=test-ipv6.com&testname=test_asn6
use crate::ipaddr::{IpType, REQUEST_AGENET};
use crate::utils::timestamp;
use crate::{err, error::ItdResult, ipaddr::{IpAddrExt,IpStateChanged}};
use crate::{BoxFuture, IpState};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

const TEST_IPV6_DOMAIN: &str = "test-ipv6.com";
#[derive(Debug, Clone)]
pub struct Ipv6Net {
    //pub url: String,
    pub frequency: i64, // 检测频率多长时间进行一次IP获取
    pub ip_types: Vec<IpType>,  // 支持IP获取类型
    // pub ip: Option<IpAddr>,
}

impl Ipv6Net {
    /// 构造函数
    /// # Arguments
    /// request_domain: 请求的域名
    pub fn new() -> Ipv6Net {
         
        Ipv6Net { 
            frequency: 300, 
            ip_types:vec![IpType::V4,IpType::V6] 
        }
    }
    pub fn get_url(&self, ip_type: IpType) -> String {
        match ip_type {
            IpType::V4 => format!(
                "https://ipv4.lookup.{}/ip/?asn=1&testdomain={}&testname=test_asn4",
                TEST_IPV6_DOMAIN, TEST_IPV6_DOMAIN
            ),
            IpType::V6 => format!(
                "https://ipv6.lookup.{}/ip/?asn=1&testdomain={}&testname=test_asn6",
                TEST_IPV6_DOMAIN, TEST_IPV6_DOMAIN
            ),
        }
    }
    pub async fn do_request(&self,url:&str) -> ItdResult<IpAddr> {
        let client = reqwest::Client::builder();
        let client = client.no_proxy().build()?;
        //let url = self.url.clone();
        let req = client.get(url);
        let req = req
            .header("Content-Type", "application/json; charset=utf-8")
            .header("User-Agent", REQUEST_AGENET);
        let resp = req.send().await?;
        let status_code = resp.status();
        let res_text = resp.text().await?;
        let res: serde_json::Value = serde_json::from_str(&res_text)?;
        if status_code != 200 {
            return err!("http status code: status_code");
        }
        // println!("res: {:?}", res);
        let ip = res["ip"].to_string().clone();
        //println!("ip: {}", ip);
        let ip = ip.as_str().trim_matches('\"');

        //println!("ip: {}", ip);
        // make sure ip is valid
        let ip = ip.parse::<IpAddr>()?;

        Ok(ip)
    }
}
impl IpAddrExt for Ipv6Net {
    fn get_ip(&self,ip_types:Vec<IpType>, ip_state: Arc<RwLock<IpState>>) -> BoxFuture<IpStateChanged> {
        Box::pin(async move {
            let mut ip_state_changed = (false, false);
            let current_timestamp = timestamp();
            for ip_type in ip_types.iter() {
                if *ip_type == IpType::V4 {

                    if !self.ip_types.contains(&IpType::V4) {
                       // ip_state_changed.0 = false;
                        continue;
                    }
                    // self.ip.unwrap().to_string();
                    let mut data = ip_state.write().await;
                    let last_updated_at = data.ipv4_updated_at;
                    if current_timestamp - last_updated_at < self.frequency {
                       // ip_state_changed.0 = false;
                        continue;
                    }
                    let url = self.get_url(IpType::V4);
                    let ip = self.do_request(&url).await?;
                    let ip_str = Some(ip.to_string());
                    if ip.is_ipv4() && data.ipv4 != ip_str {
                        data.ipv4 = ip_str;
                        data.ipv4_updated_at = current_timestamp;
                        ip_state_changed.0 = true;
                        //return Ok(true);
                    }
                    
                } else if *ip_type == IpType::V6 {

                    if !self.ip_types.contains(&IpType::V6) {
                        //ip_state_changed.0 = false;
                        continue;
                    }
                    // self.ip.unwrap().to_string();
                    let mut data = ip_state.write().await;
                    let last_updated_at = data.ipv6_updated_at;
                    if current_timestamp - last_updated_at < self.frequency {
                        //ip_state_changed.1 = false;
                        continue;
                    }
                    let url = self.get_url(IpType::V6);
                    let ip = self.do_request(&url).await?;
                    let ip_str = Some(ip.to_string());
                    if ip.is_ipv6() && data.ipv4 != ip_str {
                        data.ipv6 = ip_str;
                        data.ipv6_updated_at = current_timestamp;
                        ip_state_changed.1 = true;
                        //return Ok(true);
                    }
                    
                }
                
            }
            Ok(ip_state_changed)
       
        })
    }
    
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    #[test]
    fn it_ipaddr_parse_works() {
        let ip = "141.11.149.246".parse::<IpAddr>();
        println!("ip: {:?}", ip);
    }
}
