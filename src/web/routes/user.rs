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
        .route("/v1/user/signin", post(login))
        .route("/v1/user/create", post(create_user))
        .route("/v1/user/password-reset", post(password_reset))
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

    use crate::{error::ItdResult, utils::decode_token, web::main::tests::request};
    #[tokio::test]
    async fn it_test_login_should_work() -> ItdResult<()> {
        println!("密码错误测试Case");
        let body = r#"{"username": "admin","password": "123456"}"#;

        let response = request("/v1/user/signin", "POST", Some(body), None).await?;
        //println!("{:?}", response);
        let code = response["code"].as_i64();
        assert_eq!(code, Some(4220_i64));
        // 密码正确测试Case
        println!("密码正确测试Case");
        //assert_eq!(body.get("token").is_some(), true)

        let body = r#"{
            "username": "admin",
            "password": "Abc@1234"
        }"#;
        let response = request("/v1/user/signin", "POST", Some(body), None).await?;
        println!("{:?}", response);
        let token = response["token"].as_str();
        assert_eq!(token.is_some(), true);
        //let token = token.unwrap();
        println!("测试密码重置");
        let body = r#"{
            "old_password": "Abc@1234",
            "new_password": "Abc@1234"
        }"#;
        let response = request("/v1/user/password-reset", "POST", Some(body), token).await?;
        println!("{:?}", response);
        let result = response["code"].as_i64().unwrap();
        assert_eq!(result, 1000);
        // 添加用户测试Case
        println!("添加用户测试Case");
        let body = r#"{
            "username": "test001",
            "password": "Abc@1234",
            "repassword": "Abc@1234"
        }"#;
        let response = request("/v1/user/create", "POST", Some(body), token).await?;
        println!("{:?}", response);
        let token = response["token"].as_str();
        assert_eq!(token.is_some(), true);
        println!("删除用户测试Case");
        let token = token.unwrap();
        let claims = decode_token(token)?;
        let uid = claims.sub;
        let db = crate::get_conn().await?;
        let result = sqlx::query!(r#"delete from user where id=? "#, uid)
            .execute(&db)
            .await?;
        assert_eq!(result.rows_affected(), 1);
        Ok(())
    }
}
