use crate::{BoxFuture, IpState};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod ipv6_net;
pub mod watch;

const REQUEST_AGENET: &str = "Ip-Tell-DNS/v0.1";
#[derive(Debug, Clone,PartialEq)]
pub enum IpType {
    V4,
    V6,
}
/// (ipv4_changed, ipv6_changed)
pub type IpStateChanged = (bool, bool);
pub trait IpAddrExt {
    fn get_ip(&self,ip_types:Vec<IpType>, ip_state: Arc<RwLock<IpState>>) -> BoxFuture<IpStateChanged>;
    //fn get_record_type(&self, ip: String) -> IpAddr;
}
