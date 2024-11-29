use crate::dnspod::api_client::Client;
use crate::err;
use crate::error::ItdResult;
use serde::Deserialize;
use tracing::info;

pub struct PodAction {
    //db: &'db sqlx::Pool<sqlx::Sqlite>,
    //appid: i32,
    secret_id: String,
    secret_key: String,
    // 暂存domainlist缓存
    domain_list: Vec<DomainItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseError {
    //pub code: String,
    pub message: String,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DomainItem {
    //pub domain_id: i32,
    pub name: String,
    //pub status: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainListResponse {
    pub error: Option<ResponseError>,
    pub domain_list: Vec<DomainItem>,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RecordItem {
    pub record_id: i32,
    //pub line: String,
    //pub line_id: String,

    pub name: String,
    pub r#type: String,
    //pub updated_on: String,
    //pub value: String,
    //pub weight: Option<String>,
    //pub status: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecordListResponse {
    pub error: Option<ResponseError>,
    pub record_list: Option<Vec<RecordItem>>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddRecordResponse {
    pub error: Option<ResponseError>,
    pub record_id: Option<i32>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response<T> {
    pub response: T,
}

impl PodAction {
    /// 创建一个PodAction实例
    pub async fn new<'db>(db: &'db sqlx::Pool<sqlx::Sqlite>, appid: i32) -> ItdResult<Self> {
        let result = sqlx::query_as::<_, (String, String)>(
            "select secret_id,secret_key from user_apps where id = ?",
        )
        .bind(appid)
        .fetch_optional(db)
        .await?;
        if result.is_none() {
            return err!("appid not found");
        }
        let (secret_id, secret_key) = result.unwrap();
        Ok(PodAction {
           
            secret_id: secret_id,
            secret_key: secret_key,
            domain_list: vec![],
        })
    }
    /// 添加域名
    /// #Args
    /// * host   如果为空，则默认为@
    /// * doamin 此处domain 为不带主机名的域名
    /// * record_type 记录类型 A AAAA
    /// * ip 记录值
    /// * ttl DNS TTL
    /// #Returns
    /// * ItdResult<i32>
    ///     * 记录id
    pub async fn add_domain(
        &self,
        host: &str,
        domain: &str,
        record_type: &str,
        ip: &str,
        ttl: i32,
    ) -> ItdResult<i32> {
        /*
        let mut find_result: Option<(String, String)> = None;
        for domain_type in 1..2 {
            let hostname = self.get_hostname_from_domain(domain, domain_type);
            if let Some(hostname) = hostname {
                let left_domain = domain.strip_prefix(&(hostname.clone() + ".")).unwrap();
                let exists = self.find_domain(left_domain).await?;
                if !exists {
                    continue;
                } else {
                    find_result = Some((hostname, left_domain.to_string()));
                }
            } else {
                continue;
            }
        }*/
        let exists = self.find_domain(domain).await?;
        if !exists {
            return err!("主域名不存在，请先在DNSPod上添加主域名");
        }
        let resords = self.find_records(&domain).await?;
        for record in resords {
            //已经存在需要更新域名,注意其它的记录类型会被替换为A类型
            if record.name == host && (record.r#type == "A" || record.r#type == "AAAA") {
                self.modify_record(host, &domain, record.record_id, record_type, ip, ttl)
                    .await?;
                return Ok(record.record_id);
            }
        }
        // 添加记录
        let result = self
            .create_record(&domain, host, record_type, ip, ttl)
            .await?;
        Ok(result)
    }
    /// 增加记录
    /// # Arguments
    /// * `action` - CreateRecord | ModifyRecord`
    /// * `domain` - 域名
    /// * `hostname` - 主机名
    /// * `record_type` - 记录类型
    /// * `ip` - ip地址
    /// * `ttl` - 有效期
    pub async fn create_record(
        &self,
        hostname: &str,
        domain: &str,
        record_type: &str,
        ip: &str,
        ttl: i32,
    ) -> ItdResult<i32> {
        let client = Client::new(self.secret_id.clone(), self.secret_key.clone());

        let body = format!(
            r#"{{"Domain":"{}","SubDomain":"{}","RecordType":"{}","RecordLine":"{}","Value":"{}","TTL":{},"Status":"ENABLE"}}"#,
            domain, hostname, record_type, "默认", ip, ttl
        );
        info!("create_record ===> {}", body);
        let res = client
            .do_request::<Response<AddRecordResponse>>("POST", "CreateRecord", "", &body)
            .await?;
        if let Some(error) = res.response.error {
            return err!(error.message);
        }
        Ok(res.response.record_id.unwrap())
        // todo!()
    }
    /// 修改记录
    /// # Arguments
    /// * `domain` - 域名
    /// * `record_id` - 主机记录ID，通过 查询获取
    /// * `record_type` - 记录类型
    /// * `ip` - 记录值
    /// * `ttl` - 有效期
    ///
    pub async fn modify_record(
        &self,
        host: &str,
        domain: &str,
        record_id: i32,
        record_type: &str,
        ip: &str,
        ttl: i32,
    ) -> ItdResult<i32> {
        let client = Client::new(self.secret_id.clone(), self.secret_key.clone());

        let body = format!(
            r#"{{"Domain":"{}","RecordId":{},"RecordType":"{}","RecordLine":"{}","Value":"{}","TTL":{},"Status":"ENABLE","SubDomain":"{}"}}"#,
            domain, record_id, record_type, "默认", ip, ttl, host
        );
        info!("modify_record ===> {}", body);
        let res = client
            .do_request::<Response<AddRecordResponse>>("POST", "ModifyRecord", "", &body)
            .await?;
        if let Some(error) = res.response.error {
            return err!(error.message);
        }
        Ok(res.response.record_id.unwrap())
    }
    /// 修改记录
    /// # Arguments
    /// * `domain` - 域名
    /// * `record_id` - 主机记录ID，通过 查询获取
    ///
    pub async fn delete_record(&self, domain: &str, record_id: i32) -> ItdResult<()> {
        let client = Client::new(self.secret_id.clone(), self.secret_key.clone());

        let body = format!(r#"{{"Domain":"{}","RecordId":{}}}"#, domain, record_id);
        info!("delete_record ===> {}", body);
        let res = client
            .do_request::<Response<AddRecordResponse>>("POST", "DeleteRecord", "", &body)
            .await?;
        if let Some(error) = res.response.error {
            return err!(error.message);
        }
        Ok(())
    }
    /// 查询域名记录
    /// # Argments
    /// * domain: 域名 全域名形式带主机名 如 host.example.com
    pub async fn find_records(&self, domain: &str) -> ItdResult<Vec<RecordItem>> {
        // get domain host
        let client = Client::new(self.secret_id.clone(), self.secret_key.clone());
        let body = format!(r#"{{"Domain":"{}"}}"#, domain);
        info!("find records ===> {}", body);
        let res = client
            .do_request::<Response<RecordListResponse>>("POST", "DescribeRecordList", "", &body)
            .await?;
        if res.response.error.is_some() {
            return err!(res.response.error.unwrap().message);
        }
        Ok(res.response.record_list.unwrap())
    }
    /// 获取域名主机名
    /// # Argments
    /// * domain: 域名 全域名形式带主机名 如 host.example.com
    /// * domain_type: 域名类型 1: example.com example.cn 2: example.com.cn example.net.cn
    #[allow(dead_code)]
    pub fn get_hostname_from_domain(&self, domain: &str, domain_type: i8) -> Option<String> {
        let parts: Vec<&str> = domain.split('.').collect();

        match parts.len() {
            0 => None,
            1 => None,
            2 => Some("@".to_string()),
            3 => Some(parts[0].to_string()),
            _ => {
                //let main_domain = format!(".{}.{}", parts[parts.len()-2], parts[parts.len()-1]);
                let sub_domain = if domain_type == 1 {
                    parts[0..parts.len() - 2].join(".")
                } else {
                    parts[0..parts.len() - 3].join(".")
                };
                //parts[0..parts.len()-2].join(".");
                Some(sub_domain)
            }
        }
    }
    /// 查询域名是否已经存在
    pub async fn find_domain(&self, domain: &str) -> ItdResult<bool> {
        let domain_list = self.get_domain_list().await?;
        for item in domain_list {
            if item.name == domain {
                return Ok(true);
            }
        }
        return Ok(false);
    }
    /// 获取域名列表
    pub async fn get_domain_list(&self) -> ItdResult<Vec<DomainItem>> {
        if self.domain_list.len() > 0 {
            return Ok(self.domain_list.clone());
        }
        let client = Client::new(self.secret_id.clone(), self.secret_key.clone());
        let res = client
            .do_request::<Response<DomainListResponse>>("GET", "DescribeDomainList", "", "")
            .await?;
        //println!("===>{:?}", res);
        //let response = res.get("Response").unwrap();
        if let Some(error) = res.response.error {
            return err!(error.message);
        }
        let domain_list = res.response.domain_list;
        //self.domain_list = domain_list.clone();
        Ok(domain_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // test get_domain_list
    #[tokio::test]
    async fn test_get_domain_list() -> ItdResult<()> {
        let _db = crate::get_conn().await?;
        //let action = PodAction{db:&db};
        //let result = action.get_domain_list(1).await?;
        //println!("result: {:?}", result);
        //assert!(result.len() > 0);
        Ok(())
    }
    #[tokio::test]
    async fn test_get_host() -> ItdResult<()> {
        let domain = "www.example.com";
        let db = crate::get_conn().await?;
        let action = PodAction::new(&db, 1).await?;

        //assert_eq!(action.get_hostname_from_domain("www.example.com"), Some("www".to_string()));
        assert_eq!(
            action.get_hostname_from_domain("x.i.cloud.example.com.cn", 2),
            Some("x.i.cloud".to_string())
        );

        Ok(())
    }
    #[test]
    fn test_json_convert() -> ItdResult<()> {
        let json = r#"{
      "error":{
        "code":"InvalidParameter.Domain",
        "message":"域名不存在"
      },
      "DomainList":
        [
          {"DomainId":123456,"Name":"example.com","Status":"ENABLE"},
          {"DomainId":123457,"Name":"example.net","Status":"DISABLE"}
        ]
      }"#;
        let result: DomainListResponse = serde_json::from_str(json)?;
        println!("result: {:?}", result);
        Ok(())
    }
}
