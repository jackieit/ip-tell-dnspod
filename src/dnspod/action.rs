use crate::add_conn;
use crate::error::ItdResult;
use crate::err;
use crate::dnspod::api_client::Client;
use serde_json::Value;
use serde::Deserialize;

add_conn!(PodAction);

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

impl PodAction<'_> {
  pub async fn get_domain_list(&self,appid: &str) -> ItdResult<Vec<DomainItem>> {
    let result = sqlx::query_as::<_,(String,String)>("select secret_id,secret_key from user_apps where appid = ?")
    .bind(appid).fetch_optional(self.db).await?;
    if result.is_none(){
       return err!("appid not found");
    }
    let result = result.unwrap();
    let client = Client::new(result.0,result.1);
    let res  = client.do_request::<DomainListResponse>("GET","DescribeDomainList","","").await?;
    //let response = res.get("Response").unwrap();
    if let Some(error) = res.error{
      return err!(error.message);
    }
    let domain_list = res.domain_list;
     
    Ok(domain_list)
  } 
}