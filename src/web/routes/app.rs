use std::{ sync::Arc};

use crate::{
    error::ItdResult,
    model::{
        constants::RespMsg,
        app::{AppForm, AppItem, AppModel, StatusForm},
    },
    web::middleware::{auth::UserIdentify, validate::ValidatedData},
    AppState,
};
use axum::{extract::{State,Path}, routing::{post,put,delete}, Extension, Json, Router};

pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/app", post(create_app))
        .route("/v1/app/:id", put(update_app))
        .route("/v1/app/:id", delete(delete_app))
        .route("/v1/app/status/:id", post(set_status)) 
}
async fn create_app(
    State(app_state): State<Arc<AppState>>,
    ValidatedData(payload): ValidatedData<AppForm>,
) -> ItdResult<Json<AppItem>> {
    let db = app_state.db.clone();
    let user_model = AppModel::new(&db);
    user_model.create_app(payload).await
    //Ok(Json(result))
}
/// update app
async fn update_app(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    ValidatedData(payload): ValidatedData<AppForm>,
) -> ItdResult<Json<AppItem>> {
    let db = app_state.db.clone();
    let user_model = AppModel::new(&db);
    user_model.update_app(id, payload).await
}
/// delete app
async fn delete_app(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> ItdResult<Json<RespMsg>> {
    let db = app_state.db.clone();
    let user_model = AppModel::new(&db);
    user_model.delete_app(id).await
}
/// set app status
async fn set_status(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    ValidatedData(payload): ValidatedData<StatusForm>,
   // Extension(user_identify): Extension<UserIdentify>,
) -> ItdResult<Json<RespMsg>> {
    let db = app_state.db.clone();
    let user_model = AppModel::new(&db);
    user_model.set_status(id,  payload.status).await
}

#[cfg(test)]
mod tests {
     
    use crate::{error::ItdResult, web::main::tests::request};
    
    #[tokio::test]
    async fn it_test_app_should_work() -> ItdResult<()> {
        // create app without token
        let body = r#"{
            "uid": 1,
            "title": "test app",
            "secret_id": "test_secret_id",
            "secret_key": "test_secret_key"
        }"#;
        let response = request("/v1/app", "POST", Some(body), None).await?;
        let code = response["code"].as_i64();
        assert_eq!(code, Some(4010_i64));
        let sigin_body = r#"{
            "username": "admin",
            "password": "Abc@1234"
        }"#;
        let response = request("/v1/user/signin", "POST", Some(sigin_body), None).await?;
        println!("{:?}", response);
        let token = response["token"].as_str();
        assert_eq!(token.is_some(), true);

        let response = request("/v1/app", "POST", Some(body), token).await?;
        println!("{:?}", response);
        let app_id = response["id"].as_i64();
        assert_eq!(app_id.is_some(), true);
        // update app
        let body = r#"{
            "uid": 1,
            "title": "test app",
            "secret_id": "test_secret_id",
            "secret_key": "test_secret_key"
        }"#;
        let url  = format!("/v1/app/{}", app_id.unwrap());
        let response = request(&url, "PUT", Some(body), token).await?;
        assert_eq!(response["id"].as_i64(), app_id);
        // set app status
        let body = r#"{
            "status": 2
        }"#;
        let url  = format!("/v1/app/status/{}", app_id.unwrap());
        let response = request(&url, "POST", Some(body), token).await?;
        assert_eq!(response["code"].as_i64(), Some(1000_i64));
        // delete app
        let url  = format!("/v1/app/{}", app_id.unwrap());
        let response = request(&url, "DELETE", None, token).await?;
        assert_eq!(response["code"].as_i64(), Some(1000_i64));
        
        Ok(())
    }
}