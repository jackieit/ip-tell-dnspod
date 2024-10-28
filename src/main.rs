
mod dnspod;
mod utils;
mod error;

fn main() {
   
    println!("Hello, world!");
}

#[cfg(test)]
pub mod tests {
    use dotenv::dotenv;
    use std::env;
    use std::collections::HashMap;
    /**
     * 获取临时密钥
     */
    pub fn get_config() -> (String, String) {
        dotenv().ok();
        let env_map = env::vars()
            .into_iter()
            .map(|i| (i.0, i.1))
            .collect::<HashMap<String, String>>();
        let api_id = env_map.get("secret_id").unwrap().to_string();
        let secret_key = env_map.get("secret_key").unwrap().to_string();
        return (api_id, secret_key);
    }
}