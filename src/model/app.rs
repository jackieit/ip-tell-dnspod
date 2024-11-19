use crate::add_conn;
use crate::error::ItdResult;
use crate::model::constants::RespMsg;
use crate::utils::encrypt_data;

use axum::Json;
use chrono::Local;
use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

use validator::Validate;

add_conn!(AppModel);

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct AppItem {
    pub id: i64,
    pub uid: i64,
    pub title: String,
    pub secret_id: String,
    pub secret_key: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: Option<i64>,
}
impl<'db> AppModel<'db> {
    /// create app item
    pub async fn create_app(&self, payload: AppForm) -> ItdResult<Json<AppItem>> {
        let AppForm {
            uid,
            title,
            secret_id,
            secret_key,
        } = payload;
        let secret_id = encrypt_data(secret_id.as_bytes().to_vec())?;
        //println!("secret_id: {:?}", secret_id);
        let secret_key = encrypt_data(secret_key.as_bytes().to_vec())?;
        let result = sqlx::query!(
            r#"INSERT INTO user_apps (uid,title,secret_id,secret_key) VALUES (?,?,?,?)"#,
            uid,
            title,
            secret_id,
            secret_key
        )
        .execute(self.db)
        .await?;
        let now = Local::now().naive_local();
        Ok(Json(AppItem {
            id: result.last_insert_rowid(),
            uid,
            title,
            secret_id,
            secret_key,
            created_at: Some(now),
            updated_at: Some(now),
            status: Some(1),
        }))
    }
    /// update app item
    pub async fn update_app(&self, id: i64, payload: AppForm) -> ItdResult<Json<AppItem>> {
        let AppForm {
            uid,
            title,
            secret_id,
            secret_key,
        } = payload;
        let secret_id = encrypt_data(secret_id.as_bytes().to_vec())?;
        //println!("secret_id: {:?}", secret_id);
        let secret_key = encrypt_data(secret_key.as_bytes().to_vec())?;
        let now = Local::now().naive_local();
        sqlx::query!(
            r#"UPDATE user_apps SET uid=?,title=?,secret_id=?,secret_key=?,updated_at=? WHERE id=?"#,
            uid,
            title,
            secret_id,
            secret_key,
            now,
            id
        )
        .execute(self.db)
        .await?;
        let old = sqlx::query_as!(
            AppItem,
            r#"SELECT id,uid,title,secret_id,secret_key,created_at,updated_at,status FROM user_apps WHERE id=?"#,
            id
        )
        .fetch_one(self.db)
        .await?;
        Ok(Json(old))
    }
    /// delete app item
    pub async fn delete_app(&self, id: i64) -> ItdResult<Json<RespMsg>> {
        sqlx::query!(r#"DELETE FROM user_apps WHERE id=?"#, id)
            .execute(self.db)
            .await?;
        Ok(Json(RespMsg {
            code: Some(1000),
            message: "删除成功".to_string(),
        }))
    }
    /// set app status
    pub async fn set_status(&self, id: i64, status: i64) -> ItdResult<Json<RespMsg>> {
        sqlx::query!(r#"UPDATE user_apps SET status=? WHERE id=?"#, status, id)
            .execute(self.db)
            .await?;
        Ok(Json(RespMsg {
            code: Some(1000),
            message: "设置成功".to_string(),
        }))
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct AppForm {
    pub uid: i64,
    pub title: String,
    pub secret_id: String,
    pub secret_key: String,
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    #[test]
    pub fn it_test_now_time() {
        let now = Local::now().naive_local();
        println!("now: {:?}", now);
    }
}
