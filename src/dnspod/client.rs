/// DNSPOD API Client
/// https://www.dnspod.cn/docs/records.html
use sha2::{Sha256, Digest};
use hex;
use time::OffsetDateTime;
use time::macros::format_description;
use crate::error::{ItdResult,ItdError};
use serde::de::DeserializeOwned;

pub struct Client {
    pub api_id: String,
    pub secret_key: String,
    pub today: String,
    pub timestamp: String,
}
impl Client {
    pub fn new(api_id: String, secret_key: String) -> Client {
        let now =  OffsetDateTime::now_utc();
        let timestamp = now.unix_timestamp().to_string();
        let format = format_description!("[year]-[month]-[day]");
        let today = now.format(format).expect("Failed to format date");
        
        Client {
            api_id: api_id,
            secret_key: secret_key,
            today,
            timestamp,
        }
    }
    /// 发送请求
    /// # Args
    /// * method: 请求方法，如 GET POST PUT DELETE 等
    /// * action: 请求动作，如 DescribeInstances
    /// * query: 请求参数，如 domain=example.com&record_type=A&record_line=默认
    /// * body: 请求体，如 {"domain":"example.com","record_type":"A","record_line":"默认"}
    /// # Returns
    /// 请求结果
    pub async fn do_request<U: DeserializeOwned>(&self, method:&str, action:&str, query:&str, body:&str) -> ItdResult<U> {
        let client = reqwest::Client::new();
        let full_url = "https://dnspod.tencentcloudapi.com";
        let req_builder = match method {
            "GET" => client.get(full_url),
            "POST" => client.post(full_url),
            //"PUT" => client.put(full_url),
            //"DELETE" => client.delete(full_url),
            _ => client.get(full_url),
        };

        let authorization = "TC3-HMAC-SHA256 Credential=".to_string()
         + &self.api_id 
         + "/" + &self.today + "/" + "/dnspod/tc3_request, "
         + "SignedHeaders=content-type;host;x-tc-action, Signature=" 
         + &self.str_to_sign(method, action, query, body);

        let req_builder = if !body.is_empty() {
            req_builder.body(body.to_string())
        } else {
            req_builder
        };
    
        let req_builder = req_builder
            .header("Content-Type", "application/json")
            .header("Authorization", authorization)
            .header("Host", "dnspod.tencentcloudapi.com")
            .header("X-TC-Action",action)
            .header("X-TC-Version", "2017-03-12")
            .header("X-TC-Timestamp", &self.timestamp)
            ;
        let res = req_builder.send().await?;
        let status_code = res.status();
        let res_text = res.text().await?;
        if status_code == 200 {
            let res: U = serde_json::from_str(&res_text)?;
            
            return Ok(res);
        }else{
            return Err(ItdError::new("do_request".to_string(),status_code.to_string()));
        }
    }
    /// 签名字符串拼接
    /// https://www.dnspod.cn/docs/records.html#sign
    /// # Args
    /// * method: 请求方法，如 GET POST PUT DELETE 等
    /// * action: 请求动作，如 DescribeInstances
    /// * query: 请求参数，如 domain=example.com&record_type=A&record_line=默认
    /// * body: 请求体，如 {"domain":"example.com","record_type":"A","record_line":"默认"}
    /// # Returns
    /// 签名字符串
    pub fn canonical_request(&self,method:&str, action:&str, query:&str, body:&str) -> String {
        let request_payload_hashed = self.hashed_payload(body);
        let canonical_request = 
                    //HTTPRequestMethod
                    method.to_string() + "\n"
                    //CanonicalURI
                    + "/\n"
                    //CanonicalQueryString
                    + query + "\n"
                    //CanonicalHeaders
                    + "content-type:application/json; charset=utf-8\n"
                    + "host:dnspod.tencentcloudapi.com\n"
                    + "x-tc-action:" + action + "\n"
                    //SignedHeaders
                    + "content-type;host;x-tc-action\n"
                    //HashedRequestPayload
                    + &request_payload_hashed + "\n"
                    ;
        //let sign_str = sign_str + query + "\n";
        canonical_request
    }
    /// 计算签名字符串
    /// # Args
    /// * method: 请求方法，如 GET POST PUT DELETE 等
    /// * action: 请求动作，如 DescribeInstances
    /// * query: 请求参数，如 domain=example.com&record_type=A&record_line=默认
    /// * body: 请求体，如 {"domain":"example.com","record_type":"A","record_line":"默认"}
    /// # Returns
    /// 签名字符串
    pub fn str_to_sign(&self,method:&str, action:&str, query:&str, body:&str) -> String {
        let canonical_request = self.canonical_request(method, action, query, body);
        //let date = chrono::Local::now().format("%Y-%m-%d").to_string();
          
        let service = "dnspod";
        //let region = "ap-guangzhou";
        let algorithm = "TC3-HMAC-SHA256".to_string();
        let sign_str = 
            algorithm + "\n"
            + &self.timestamp + "\n"
            + &self.today +"/"+service+"/tc3_request"+ "\n"
            + &self.hashed_payload(&canonical_request)
            ;
        let secret_key = "TC3".to_string() + &self.secret_key;
        // hashmac sha256 hash date
        let secret_date = self.hmac_sha256(&secret_key,&self.today);
        let secret_service = self.hmac_sha256(&secret_date,service);
        let secret_signing = self.hmac_sha256(&secret_service, "tc3_request");
        
        self.hmac_sha256(&secret_signing, &sign_str) 
    }
    /// 计算请求体哈希值
    /// # Args
    /// * body: 请求体，如 {"domain":"example.com","record_type":"A","record_line":"默认"}
    /// # Returns
    /// 哈希值
    pub fn hashed_payload(&self,body:&str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(body);
        let result = hasher.finalize();
        hex::encode(result)
    }
    /// 计算哈希值
    /// 哈希值
    pub fn hmac_sha256(&self,key: &str, data:&str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key);
        let result = hasher.finalize();
        hex::encode(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    #[tokio::test]
    async fn test_client() -> Result<(), ItdError> {
        let (api_id, secret_key) = crate::tests::get_config();
        let client = Client::new(api_id, secret_key);
        let result =  client.do_request::<Value>("GET","DescribeDomainList","", "{\"Offset\":0,\"Length\":20}").await?;
        println!("{:?}", result);
        Ok(())
    }
}