use std::net::IpAddr;
pub mod ipv6_net;

const REQUEST_AGENET: &str = "Ip-Tell-DNS/v0.1";
#[derive(Debug, Clone)]
pub enum IpType {
    V4,
    V6,
}
pub trait IpAddrExt {
    fn get_ip(&self, ip_type: IpType) -> String;
    fn get_record_type(&self, ip: String) -> IpAddr;
}
