use std::{path::Path, sync::Arc};

use crate::{
    error::ItdResult,
    model::{
        constants::RespMsg,
        app::{AppForm, AppItem, AppModel, StatusForm},
    },
    web::middleware::{auth::UserIdentify, validate::ValidatedData},
    AppState,
};
use axum::{extract::State, routing::{post,put,delete}, Extension, Json, Router};

pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/app", post(create_app))
        .route("/v1/app/:id", put(update_user))
        .route("/v1/app/:id", delete(delete_use))
        .route("/v1/app/status", post(set_status)) 
}
async fn create_app(
    State(app_state): State<Arc<AppState>>,
    ValidatedData(payload): ValidatedData<AppForm>,
) -> ItdResult<Json<AppItem>> {
    let db = app_state.db.clone();
    let user_model = AppModel::new(&db);
    user_model.create_app(payload).await?
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
    user_model.update_app(id, payload).await?
}
/// delete app
async fn delete_app(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> ItdResult<Json<RespMsg>> {
    let db = app_state.db.clone();
    let user_model = AppModel::new(&db);
    user_model.delete_app(id).await?
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
    user_model.set_status(id,  payload.status).await?
}
