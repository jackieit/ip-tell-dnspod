use crate::add_conn;
use crate::error::ItdResult;
use axum::Json;
use chrono::NaiveDateTime;
use crate::model::constants::Pagination;
use validator::Validate;

add_conn!(Records);
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Record {
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub secret_id: Option<String>,
    pub secret_key: Option<String>,
}
impl<'db> Records<'db> {
    /// get all records
 
    pub async fn get_record_list(&self) -> ItdResult<Vec<Record>> {
        let record_list: Vec<Record> = sqlx::query_as(
            r#"SELECT 
                     i.id ,appid,host,domain,ip_type,ip,weight,ttl,i.created_at,i.updated_at
                     ,i.record_id
                     ,ii.secret_id,ii.secret_key
                     FROM user_domain i left join user_apps ii on i.appid=ii.id
                    "#,
        )
        .fetch_all(self.db)
        .await?;
        Ok(record_list)
    }
    /// search by appid and return 
    pub async fn search(&self,appid: Option<i32>, page: i32) -> ItdResult<Json<Pagination<Record>>> {
        let offset = (page - 1) * 10;
        let sql_base = r#"SELECT 
                     i.id ,appid,host,domain,ip_type,ip,weight,ttl,i.created_at,i.updated_at
                     ,i.record_id
                     ,ii.secret_id,ii.secret_key
                     FROM user_domain i left join user_apps ii on i.appid=ii.id "#;
        let sql_total = if let Some(appid) = appid {
            format!("{} where i.appid= {}",sql_base,appid)
        }else{
            sql_base.to_string()
        };
        let total: i32 = sqlx::query_scalar(&sql_total)
        .bind(appid)
        .fetch_one(self.db)
        .await?;
        let sql_query = if let Some(appid) = appid {
            format!("{} where i.appid={} limit {},10",sql_base,appid,offset)
        }else{
            format!("{} limit {},10",sql_base,offset)
        };
        let record_list: Vec<Record> = sqlx::query_as(&sql_query)
        .bind(appid)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        let pagination = Json(Pagination {
            count: total,
            page,
            data: record_list,
        });
        Ok(pagination)
    }
    /// get one record
    pub async fn get_record(&self, record_id: i64) -> ItdResult<Option<Record>> {
        let record: Option<Record> = sqlx::query_as(
            r#"SELECT 
             i.id ,appid,host,domain,ip_type,ip,weight,ttl,i.created_at,i.updated_at
             ,i.record_id
             ,ii.secret_id,ii.secret_key
             FROM user_domain i left join user_apps ii on i.appid=ii.id
             where i.id=?
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
             i.id ,appid,host,domain,ip_type,ip,weight,ttl,i.created_at,i.updated_at
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
            host,
            domain,
            ip,
            ip_type,
            weight,
            record_id,
            ttl,
        } = payload;

        let result = sqlx::query!(
            r#"
            INSERT INTO user_domain (appid,host,domain,ip,ip_type,weight,record_id,ttl)
            VALUES (?,?,?,?,?,?,?,?)
            "#,
            appid,
            host,
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
    pub async fn update_record(&self, id: i64, payload: RecordForm) -> ItdResult<u64> {
        let RecordForm {
            appid,
            host,
            domain,
            ip,
            ip_type,
            weight,
            record_id,
            ttl,
        } = payload;

        let result = sqlx::query!(r#"UPDATE user_domain SET appid = ?, host = ?, domain = ?, ip = ?, ip_type = ?, weight = ?, record_id = ?, ttl = ? WHERE id = ?"#,
            appid,
            host,
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
    pub async fn delete_record(&self, record_id: i64) -> ItdResult<u64> {
        let result = sqlx::query!(r#"delete from user_domain where record_id = ?"#, record_id)
            .execute(self.db)
            .await?;
        Ok(result.rows_affected())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecordForm {
    pub appid: i32,
    pub host: String,
    pub domain: String,
    pub ip: String,
    pub ip_type: String,
    pub weight: Option<i32>,
    pub record_id: Option<i32>,
    pub ttl: i32,
}
#[derive(Deserialize, Debug, Validate)]
pub struct QueryForm {
    pub page: Option<i32>,
    pub appid: Option<i32>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dnspod::action::PodAction;
    use crate::get_conn;
    /// 添加域名测试
    #[tokio::test]
    async fn add_domain_test() -> ItdResult<()> {
        let db = get_conn().await?;
        let records = Records::new(&db);
        let ip = "27.214.7.126".to_string();

        let action = PodAction::new(&db, 1).await?;
        let record_id = action.add_domain("itd", "guoran.cn", "A", &ip, 600).await?;
        let data_record = RecordForm {
            appid: 1,
            host: "itd".to_string(),
            domain: "guoran.cn".to_string(),
            ip: ip.clone(),
            ip_type: "A".to_string(),
            weight: None,
            record_id: Some(record_id),
            ttl: 600,
        };
        let result = records.create_record(data_record).await?;
        assert_eq!(result > 0, true);
        Ok(())
    }
    /// 删除域名
    #[tokio::test]
    async fn delete_domain_test() -> ItdResult<()> {
        let db = get_conn().await?;
        let action = PodAction::new(&db, 1).await?;
        let result = action.delete_record("guoran.cn", 1884171111).await?;
        assert_eq!(result, ());
        Ok(())
    }
    /// 获取所有记录
    #[tokio::test]
    async fn test_get_records_list() -> ItdResult<()> {
        let db = get_conn().await?;
        let action = PodAction::new(&db, 1).await?;
        let record_list = action.find_records("guoran.cn").await?;
        //println!("record_list is : {:?}", record_list);
        for record in record_list {
            if record.name == "itd" {
                println!("record is : {:?}", record);
                let result = action
                    .modify_record("ltd", "guoran.cn", 1884171111, "A", "27.214.7.125", 600)
                    .await?;
                assert_eq!(result > 0, true);
            }
        }
        Ok(())
    }
}
