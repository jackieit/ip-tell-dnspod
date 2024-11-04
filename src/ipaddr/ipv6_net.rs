use crate::{
    err,
    error::ItdResult,
    ipaddr::{IpAddrExt, IpType},
};

#[derive(Debug, Clone)]
pub struct Ipv6Net {
    pub request_domain: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}

/// 请求网站 test-ipv6.com
/// 获取ipv4 https://ipv4.lookup.test-ipv6.com/ip/?asn=1&testdomain=test-ipv6.com&testname=test_asn4
/// 获取ipv6 https://ipv6.lookup.test-ipv6.com/ip/?asn=1&testdomain=test-ipv6.com&testname=test_asn6
impl Ipv6Net {
    /// 构造函数
    /// # Arguments
    /// request_domain: 请求的域名
    pub fn new(request_domain: String) -> Ipv6Net {
        Ipv6Net {
            request_domain,
            ipv4: None,
            ipv6: None,
        }
    }
    pub fn get_url(&self, ip_type: IpType) -> String {
        match ip_type {
            IpType::IpV4 => format!(
                "https://ipv4.lookup.{}/ip/?asn=1&testdomain={}&testname=test_asn4",
                self.request_domain, self.request_domain
            ),
            IpType::IpV6 => format!(
                "https://ipv6.lookup.{}/ip/?asn=1&testdomain={}&testname=test_asn6",
                self.request_domain, self.request_domain
            ),
        }
    }
    pub async fn do_request(&mut self, ip_type: IpType) -> ItdResult<String> {
        let client = reqwest::Client::new();
        let url = self.get_url(ip_type.clone());
        let req = client.get(url);
        let req = req
            .header("Content-Type", "application/json; charset=utf-8")
            .header("User-Agent", "ip-tell-dns");
        let resp = req.send().await?;
        let status_code = resp.status();
        let res_text = resp.text().await?;
        let res: serde_json::Value = serde_json::from_str(&res_text)?;
        if status_code != 200 {
            return err!("http status code: status_code");
        }
        let ip = res["ip"].to_string();
        match ip_type {
            IpType::IpV4 => {
                self.ipv4 = Some(ip.clone());
                return Ok(ip);
            }
            IpType::IpV6 => {
                self.ipv6 = Some(ip.clone());
                return Ok(ip);
            }
        }
    }
}
impl IpAddrExt for Ipv6Net {
    fn get_ip(&self, ip_type: IpType) -> String {
        match ip_type {
            IpType::IpV4 => self.ipv4.clone().unwrap(),
            IpType::IpV6 => self.ipv6.clone().unwrap(),
            _ => panic!("ip_type error"),
        }
    }
    fn get_record_type(&self) -> String {
        match self.ipv4 {
            Some(_) => "A".to_string(),
            None => "AAAA".to_string(),
        }
    }
}
