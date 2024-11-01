use crate::add_conn;
use crate::error::ItdResult;
use crate::err;
use crate::dnspod::api_client::Client;
use serde::Deserialize;

pub struct PodAction<'db> {
  db: &'db sqlx::Pool<sqlx::Sqlite>,
  //appid: i32,
  secret_id: String,
  secret_key: String
}

 

#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResponseError {
  pub code: String,
  pub message: String,
}
#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainItem {
  pub domain_id: i32,
  pub name: String,
  pub status: String,
}
#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DomainListResponse {
  pub error: Option<ResponseError>,
  pub domain_list: Vec<DomainItem>,
}
#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response<T> {
  pub response: T,
}

impl <'db>PodAction<'db> {
  /// 创建一个PodAction实例
  pub async fn new(db: &'db sqlx::Pool<sqlx::Sqlite>,appid: i32) -> ItdResult<Self> {
    let result = sqlx::query_as::<_,(String,String)>(
      "select secret_id,secret_key from user_apps where id = ?")
    .bind(appid).fetch_optional(db).await?;
    if result.is_none(){
       return err!("appid not found");
    }
    let (secret_id,secret_key) = result.unwrap();
    Ok(PodAction { db ,secret_id: secret_id,secret_key: secret_key })
  }
  /// 查询域名记录
  /// # Argments
  /// * domain: 域名 全域名形式带主机名 如 host.example.com
  pub async fn find_records(&self,domain: &str) -> ItdResult<Vec<(String,String)>> {
    // get domain host
    
    todo!()
  }
  /// 获取域名主机名
  /// # Argments
  /// * domain: 域名 全域名形式带主机名 如 host.example.com
  /// * domain_type: 域名类型 1: example.com example.cn 2: example.com.cn example.net.cn
  pub fn get_hostname_from_domain(&self, domain: &str ,domain_type: i8) -> Option<String> {
    let parts: Vec<&str> = domain.split('.').collect();
    
    match parts.len() {
      0 => None,
      1 => None,
      2 => Some("@".to_string()),
      3 => Some(parts[0].to_string()),
      _ => {
        //let main_domain = format!(".{}.{}", parts[parts.len()-2], parts[parts.len()-1]);
        let sub_domain = 
        if domain_type == 1 {
          parts[0..parts.len()-2].join(".")
        } else {
          parts[0..parts.len()-3].join(".")
        };
        //parts[0..parts.len()-2].join(".");
        Some(sub_domain)
      }
    }
  }
  /// 查询域名是否已经存在
  pub async fn find_domain(&self,domain: &str) -> ItdResult<bool> {
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
    
    let client = Client::new(self.secret_id.clone(),self.secret_key.clone());
    let res  = client.do_request::<Response<DomainListResponse>>("GET","DescribeDomainList","","").await?;
    //let response = res.get("Response").unwrap();
    if let Some(error) = res.response.error {
      return err!(error.message);
    }
    let domain_list = res.response.domain_list;
     
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
  async fn test_get_host() -> ItdResult<()>{
    let domain = "www.example.com";
    let db = crate::get_conn().await?;
    let action = PodAction::new(&db,1).await?;
   
    //assert_eq!(action.get_hostname_from_domain("www.example.com"), Some("www".to_string()));
    assert_eq!(action.get_hostname_from_domain("x.i.cloud.example.com.cn",2), Some("x.i.cloud".to_string()));

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