///! 请求网站 test-ipv6.com
///！ 获取ipv4 https://ipv4.lookup.test-ipv6.com/ip/?asn=1&testdomain=test-ipv6.com&testname=test_asn4
///！ 获取ipv6 https://ipv6.lookup.test-ipv6.com/ip/?asn=1&testdomain=test-ipv6.com&testname=test_asn6
use crate::ipaddr::{IpType, REQUEST_AGENET};
use crate::{err, error::ItdResult, ipaddr::IpAddrExt};
use std::net::IpAddr;
#[derive(Debug, Clone)]
pub struct Ipv6Net {
    pub url: String,
    pub ip: Option<IpAddr>,
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
        Ipv6Net { url, ip: None }
    }

    pub async fn do_request(&mut self) -> ItdResult<IpAddr> {
        let client = reqwest::Client::new();
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
        let ip = res["ip"].to_string();
        let ip: IpAddr = ip.parse().unwrap();
        Ok(ip)
    }
}
impl IpAddrExt for Ipv6Net {
    fn get_ip(&self, ip_type: IpType) -> String {
        self.ip.unwrap().to_string()
    }
    fn get_record_type(&self, ip: String) -> IpAddr {
        let ip: IpAddr = ip.parse().unwrap();
        ip
    }
}
