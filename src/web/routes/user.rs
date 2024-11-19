use std::sync::Arc;

use crate::{
    error::ItdResult,
    model::{
        constants::RespMsg,
        user::{LoginForm, PasswordForm, SignupForm, UserModel, UserToken},
    },
    web::middleware::{auth::UserIdentify, validate::ValidatedData},
    AppState,
};
use axum::{extract::State, routing::post, Extension, Json, Router};

pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/user/signin", post(login))
        .route("/user/create", post(create_user))
        .route("/user/password-reset", post(password_reset))
    //.route("/user/is-admin", post(is_admin))
}

/// 用户登录
async fn login(
    State(app_state): State<Arc<AppState>>,
    ValidatedData(payload): ValidatedData<LoginForm>,
) -> ItdResult<Json<UserToken>> {
    let db = app_state.db.clone();
    let user_model = UserModel::new(&db);
    let result = user_model.login(payload).await?;
    Ok(Json(result))
}
async fn create_user(
    State(app_state): State<Arc<AppState>>,
    ValidatedData(payload): ValidatedData<SignupForm>,
) -> ItdResult<Json<UserToken>> {
    let db = app_state.db.clone();
    let user_model = UserModel::new(&db);
    let result = user_model.create_user(payload).await?;
    Ok(Json(result))
}
/// 修改密码
async fn password_reset(
    Extension(user_identify): Extension<UserIdentify>,
    State(app_state): State<Arc<AppState>>,
    ValidatedData(payload): ValidatedData<PasswordForm>,
) -> ItdResult<Json<RespMsg>> {
    let db = app_state.db.clone();
    let user_model = UserModel::new(&db);
    let result = user_model.password_reset(user_identify.id, payload).await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::web::main::test_app;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;
    #[tokio::test]
    async fn it_test_login_should_work() {
        let app = test_app().await;
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/user/signin")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::USER_AGENT, "tokio-test")
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "username": "admin",
                            "password": "Abc@123"
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        //assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        println!("response ===>{}", body);
        assert_eq!(body.get("code"), Some(&Value::Number(40022.into())));

        assert_eq!(body.get("token").is_some(), true)
    }
}
