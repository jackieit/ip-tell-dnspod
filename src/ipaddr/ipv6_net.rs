///! 请求网站 test-ipv6.com
///！ 获取ipv4 https://ipv4.lookup.test-ipv6.com/ip/?asn=1&testdomain=test-ipv6.com&testname=test_asn4
///！ 获取ipv6 https://ipv6.lookup.test-ipv6.com/ip/?asn=1&testdomain=test-ipv6.com&testname=test_asn6
use crate::ipaddr::{IpType, REQUEST_AGENET};
use crate::utils::timestamp;
use crate::{err, error::ItdResult, ipaddr::IpAddrExt};
use crate::{BoxFuture, IpState};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
#[derive(Debug, Clone)]
pub struct Ipv6Net {
    pub url: String,
    // pub ip: Option<IpAddr>,
}

impl Ipv6Net {
    /// 构造函数
    /// # Arguments
    /// request_domain: 请求的域名
    pub fn new(request_domain: String, ip_type: IpType) -> Ipv6Net {
        let url = match ip_type {
            IpType::V4 => format!(
                "https://ipv4.lookup.{}/ip/?asn=1&testdomain={}&testname=test_asn4",
                request_domain, request_domain
            ),
            IpType::V6 => format!(
                "https://ipv6.lookup.{}/ip/?asn=1&testdomain={}&testname=test_asn6",
                request_domain, request_domain
            ),
        };
        Ipv6Net { url }
    }

    pub async fn do_request(&self) -> ItdResult<IpAddr> {
        let client = reqwest::Client::builder();
        let client = client.no_proxy().build()?;
        let url = self.url.clone();
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
    fn get_ip(&self, ip_state: Arc<Mutex<IpState>>) -> BoxFuture<bool> {
        Box::pin(async move {
            let ip = self.do_request().await?;
            // self.ip.unwrap().to_string();
            if let Ok(mut data) = ip_state.lock() {
                let ip_str = Some(ip.to_string());
                if ip.is_ipv4() && data.ipv4 != ip_str {
                    data.ipv4 = ip_str;
                    data.ipv4_updated_at = timestamp();
                    return Ok(true);
                }
                if ip.is_ipv6() && data.ipv6 != ip_str {
                    data.ipv6 = ip_str;
                    data.ipv6_updated_at = timestamp();
                    return Ok(true);
                }
                if data.ipv6 == ip_str || data.ipv4 == ip_str {
                    return Ok(false);
                }
            }

            Ok(false)
        })
    }
    fn get_record_type(&self, ip: String) -> IpAddr {
        let ip: IpAddr = ip.parse().unwrap();
        ip
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
