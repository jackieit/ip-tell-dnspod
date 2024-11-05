use crate::error::ItdResult;
use chrono::NaiveDateTime;
pub struct Records<'db> {
    pub db: &'db sqlx::Pool<sqlx::Sqlite>,
}
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Record {
    pub id: i32,
    pub appid: i32,
    pub domain: String,
    pub ip_type: i8,
    /// 原始记录id来源于Dnspod
    pub record_id: String,
    pub ip: String,
    pub weight: i32,
    pub ttl: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub secret_id: Option<String>,
    pub secret_key: Option<String>,
}
impl<'db> Records<'db> {
    pub fn new(db: &'db sqlx::Pool<sqlx::Sqlite>) -> Self {
        Records { db }
    }
    /// get all records
    pub async fn get_record_list(&self) -> ItdResult<Vec<Record>> {
        let record_list: Vec<Record> = sqlx::query_as(
            r#"SELECT 
             i.id ,appid,domain,ip_type,ip,weight,ttl,i.created_at,i.updated_at
             ,i.record_id
             ,ii.secret_id,ii.secret_key
             FROM user_domain i left join user_apps ii on i.appid=ii.id
            "#,
        )
        .fetch_all(self.db)
        .await?;
        Ok(record_list)
    }
    /// get one record
    pub async fn get_record(&self, record_id: i32) -> ItdResult<Option<Record>> {
        let record: Option<Record> = sqlx::query_as(
            r#"SELECT 
             i.id ,appid,domain,ip_type,ip,weight,ttl,i.created_at,i.updated_at
             ,i.record_id
             ,ii.secret_id,ii.secret_key
             FROM user_domain i left join user_apps ii on i.appid=ii.id
             where i.id=?1
            "#,
        )
        .bind(record_id)
        .fetch_optional(self.db)
        .await?;
        Ok(record)
    }
    /// get one recored by domain
    pub async fn get_record_by_domain(&self, domain: &str) -> ItdResult<Vec<Record>> {
        let record_list: Vec<Record> = sqlx::query_as(
            r#"SELECT 
             i.id ,appid,domain,ip_type,ip,weight,ttl,i.created_at,i.updated_at
             ,i.record_id
             ,ii.secret_id,ii.secret_key
             FROM user_domain i left join user_apps ii on i.appid=ii.id
             where i.domain=?
            "#,
        )
        .bind(domain)
        .fetch_all(self.db)
        .await?;
        Ok(record_list)
    }
    pub async fn create_record(&self, payload: RecordForm) -> ItdResult<i64> {
        let RecordForm {
            appid,
            domain,
            ip,
            ip_type,
            weight,
            record_id,
            ttl,
        } = payload;
        let result = sqlx::query!(
            r#"
            INSERT INTO user_domain (appid,domain,ip,ip_type,weight,record_id,ttl)
            VALUES (?,?,?,?,?,?,?)
            "#,
            appid,
            domain,
            ip,
            ip_type,
            weight,
            record_id,
            ttl
        )
        .execute(self.db)
        .await?;
        Ok(result.last_insert_rowid())
    }
    /// update record
    pub async fn update_record(&self, id: i32, payload: RecordForm) -> ItdResult<u64> {
        let RecordForm {
            appid,
            domain,
            ip,
            ip_type,
            weight,
            record_id,
            ttl,
        } = payload;
        let result = sqlx::query!(r#"UPDATE user_domain SET appid = ?, domain = ?, ip = ?, ip_type = ?, weight = ?, record_id = ?, ttl = ? WHERE id = ?"#,
            appid,
            domain,
            ip,
            ip_type,
            weight,
            record_id,
            ttl,
            id)
            .execute(self.db)
            .await?;
        Ok(result.rows_affected())
    }
    /// delete record
    pub async fn delete_record(&self, record_id: i32) -> ItdResult<u64> {
        let result = sqlx::query!(r#"delete from user_domain where record_id = ?"#, record_id)
            .execute(self.db)
            .await?;
        Ok(result.rows_affected())
    }
}

#[derive(Debug, Deserialize)]
pub struct RecordForm {
    pub appid: i32,
    pub domain: String,
    pub ip: String,
    pub ip_type: String,
    pub weight: Option<i32>,
    pub record_id: Option<i32>,
    pub ttl: i32,
}
