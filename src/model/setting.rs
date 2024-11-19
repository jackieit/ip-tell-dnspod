use crate::add_conn;
use crate::err;
use crate::error::ItdResult;
use std::collections::HashMap;

add_conn!(SettingModel);

impl<'db> SettingModel<'db> {
    // query setting BY KEY
    pub async fn get(&self, key: &str) -> ItdResult<String> {
        let setting = sqlx::query_as::<_, (String,)>("SELECT value FROM setting WHERE `key` = ?")
            .bind(key)
            .fetch_optional(self.db)
            .await?;

        if let Some(setting) = setting {
            return Ok(setting.0);
        }
        err!("setting NOT found")
    }
    // query ALL BY keys
    pub async fn get_all(&self, key: &str) -> ItdResult<HashMap<String, String>> {
        let key_like = format!("%{}%", key);
        let settings = sqlx::query_as::<_, (String, String)>(
            "SELECT `key`,value FROM setting WHERE `key` LIKE ?",
        )
        .bind(key_like)
        .fetch_all(self.db)
        .await?;
        let result: HashMap<String, String> = settings.into_iter().collect();

        Ok(result)
    }
    // INSERT setting
    pub async fn set(&self, key: &str, val: &str, title: &str) -> ItdResult<bool> {
        let setting = sqlx::query_as::<_, (String,)>("SELECT value FROM setting WHERE `key` = ?")
            .bind(key)
            .fetch_optional(self.db)
            .await?;

        if setting.is_some() {
            sqlx::query!("UPDATE setting SET value = ? WHERE `key` = ?", val, key)
                .execute(self.db)
                .await?;
        } else {
            sqlx::query!(
                "INSERT INTO setting(`key`,`value`,description) VALUES(?,?,?)",
                key,
                val,
                title
            )
            .execute(self.db)
            .await?;
        }

        Ok(true)
    }
    // DELETE setting
    pub async fn delete(&self, key: &str) -> ItdResult<bool> {
        sqlx::query!("delete FROM setting WHERE `key` = ?", key)
            .execute(self.db)
            .await?;

        return Ok(true);
    }

    // request aliyun send sms service
}
#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_insert_works() {}
    #[tokio::test]
    async fn test_get_all_works() {}
}
