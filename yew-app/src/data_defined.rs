use serde::{Deserialize, Serialize};
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct RecordListItem {
    pub id: i32,
    pub appid: i32,
    pub host: String,
    pub domain: String,
    pub ip_type: String,
    /// 原始记录id来源于Dnspod
    pub record_id: i32,
    pub ip: String,
    pub weight: i32,
    pub ttl: i32,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PagationRecords {
    pub count: i32,
    pub page: i32,
    pub data: Vec<RecordListItem>,
}