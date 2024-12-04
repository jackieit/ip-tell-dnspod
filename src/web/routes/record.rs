use std::sync::Arc;

use crate::{
    error::ItdResult, model::{
        constants::{Pagination, RespMsg},
        records::{QueryForm, Record, RecordForm, Records},
    }, utils::extract_ip, web::middleware::validate::ValidatedData, AppState
};
use axum::{extract::{Path, Query, State}, routing::{ delete, get, post, put }, Json, Router};

pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/records", get(list_all))
        .route("/v1/record", post(create_record))
        .route("/v1/record/:id", get(view_record))
        .route("/v1/record/:id", put(update_record)) 
        .route("/v1/record/:id", delete(delete_record))
}
async fn list_all(
    State(state): State<Arc<AppState>>,
    Query(payload): Query<QueryForm>,
) -> ItdResult<Json<Pagination<Record>>> {
    let record_model = Records::new(&state.db);
    let QueryForm{appid,page  } = payload;
    let page = page.unwrap_or(1);
    record_model.search(appid, page).await
}
async fn create_record(
    State(state): State<Arc<AppState>>,
    ValidatedData(payload): ValidatedData<RecordForm>,
) -> ItdResult<Json<Option<Record>>> {
    let record_model = Records::new(&state.db);
    let mut data = payload.clone();
    if payload.ip.is_none() {
         
        let app_state = state.clone();
        //let ip_state = app_state.ip_state.lock().unwrap();
        let ip_value =  extract_ip(&payload.ip_type,app_state).await?;
      data.ip = Some(ip_value);
    }
    
    let last_insert_id = record_model.create_record(data).await?;

    let record = record_model.get_record(last_insert_id).await?;
    //Ok(Redirect::temporary("/v1/record/".to_string() + &last_insert_id.to_string()))
    Ok(Json(record))
}
async fn view_record(
    State(state): State<Arc<AppState>>,
    Path(record_id): Path<i64>,
) -> ItdResult<Json<Record>> {
    let record_model = Records::new(&state.db);
    let record = record_model.get_record(record_id).await?;
    let record = record.unwrap();
    Ok(Json(record))
}
async fn update_record(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    ValidatedData(payload): ValidatedData<RecordForm>,
) -> ItdResult<Json<Option<Record>>> {
    let record_model = Records::new(&state.db);
    let mut data = payload.clone();
    if payload.ip.is_none() {
         
      let app_state = state.clone();
      //let ip_state = app_state.ip_state.lock().unwrap();
      let ip_value =  extract_ip(&payload.ip_type,app_state).await?;
      data.ip = Some(ip_value);
    }
    let _ = record_model.update_record(id, data).await?;
    let record = record_model.get_record(id).await?;
    //let record = record.unwrap();
    Ok(Json(record))
}
async fn delete_record(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> ItdResult<Json<RespMsg>> {
    let record_model = Records::new(&state.db);
    let _ = record_model.delete_record(id).await?;
    Ok(Json(RespMsg { code: Some(1000), message: "Ok".to_string() }))
}

#[cfg(test)]
mod tests {

    use crate::{error::ItdResult, web::main::tests::request};
    #[tokio::test]
    async fn it_test_record_should_work() -> ItdResult<()> {
      // create app without token
      let sigin_body = r#"{
          "username": "admin",
          "password": "Abc@1234"
      }"#;
      let response = request("/v1/user/signin", "POST", Some(sigin_body), None).await?;
      println!("{:?}", response);
      let token = response["token"].as_str();
      assert_eq!(token.is_some(), true);
      let body = r#"{
        "appid": 1,
        "host": "itd",
        "domain": "guoran.cn",
        "ip":null,
        "ip_type":"A",
        "weight": 1,
        "record_id":null,
        "ttl": 600
      }"#;
      let response = request("/v1/record", "POST", Some(body), token).await?;
      println!("{:?}", response);
      let app_id = response["id"].as_i64();
      assert_eq!(app_id.is_some(), true);
      Ok(())
    }
  }