use crate::error::{AuthenticateError, ItdError};

use crate::utils::decode_token;
use crate::AppState;
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};

use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    //pub id: i32,
}

#[derive(Clone)]
pub struct UserIdentify {
    pub id: i64,
    // pub user: Option<UserModel>,
}
pub async fn auth(
    State(app_state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ItdError> {
    let path = req.uri().path().to_string();
    let cmp_path = path.clone();
    let method = req.method().clone();
    if cmp_path == "/user/signin"
        || cmp_path.starts_with("/user/signup")
        || cmp_path.starts_with("/utils")
        || cmp_path.starts_with("/assets")
        || cmp_path.starts_with("/h5")
        || cmp_path == "/"
        || cmp_path == "/index.html"
        || method == http::Method::OPTIONS
    {
        return Ok(next.run(req).await);
    }
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .map(|auth_header| match auth_header.split_once(" ") {
            Some((name, token)) if name == "Bearer" => token.to_owned(),
            _ => "".to_owned(),
        })
        .unwrap_or("".to_owned());
    if token.is_empty() {
        return Err(ItdError::from(AuthenticateError::WrongCredentials));
    }

    let token_message = decode_token(&token).unwrap();
    let uid = token_message.sub;

    //println!("request auth check ===> {},{},{}", &path, &action, &token);
    //let token = bearer.token().to_owned();
    let conn = app_state.db.clone();
    let exists: (bool,) =
        sqlx::query_as::<_, (bool,)>("SELECT EXISTS(SELECT id FROM user WHERE id = ?)")
            .bind(uid)
            .fetch_one(&conn)
            .await?;

    if !exists.0 {
        return Err(ItdError::from(AuthenticateError::WrongCredentials));
    } else {
        //let Some((user_model, profile_model)) = user_token_model;

        let current_user = UserIdentify { id: uid };
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    }
}
