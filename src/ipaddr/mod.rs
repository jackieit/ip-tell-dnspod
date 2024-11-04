mod ipv6_net;

#[derive(Debug, Clone)]
pub enum IpType {
    IpV4,
    IpV6,
}
pub trait IpAddrExt {
    fn get_ip(&self, ip_type: IpType) -> String;
    fn get_record_type(&self) -> String;
}
