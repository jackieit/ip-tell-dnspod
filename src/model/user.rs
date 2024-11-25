use super::constants::{RespMsg, USERNAME_REGEX};
use crate::add_conn;
use crate::err;

use crate::error::ItdResult;
use crate::utils::{encode_token, password_hash, verify_password};
use axum::Json;

use serde::{Deserialize, Serialize};

use validator::Validate;

add_conn!(UserModel);
pub struct UserRow {
    pub id: Option<i64>,
    pub password: String,
    pub status: Option<i64>,
}

pub struct UserPasswordRow {
    pub password: String,
    pub username: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserToken {
    token: String,
}
impl<'db> UserModel<'db> {
    /// 用户登录
    pub async fn login(&self, payload: LoginForm) -> ItdResult<UserToken> {
        let username = payload.username;
        //let mobile_secure = encrypt_data(mobile.as_bytes().to_vec())?;
        let user_row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, password, status FROM user WHERE username = ?"#,
            username
        )
        .fetch_optional(self.db)
        .await?;
        if user_row.is_none() {
            return err!("用户不存在");
        }
        let user_row = user_row.unwrap();
        if user_row.status != Some(1) {
            return err!("用户状态异常");
        }
        if !verify_password(&payload.password, &user_row.password) {
            return err!("密码错误");
        }

        let access_token = encode_token(user_row.id.unwrap(), 3600 * 24 * 7)?;
        Ok(UserToken {
            token: access_token.0,
        })
    }
    /// 用户添加
    pub async fn create_user(&self, payload: SignupForm) -> ItdResult<UserToken> {
        println!("signup payload: {:?}", payload);
        let username = payload.username;
        // let mobile_secure = encrypt_data(mobile.as_bytes().to_vec())?;
        let user_row = sqlx::query_as!(
            UserRow,
            r#"SELECT id, password, status FROM user WHERE username = ?"#,
            username
        )
        .fetch_optional(self.db)
        .await?;
        if user_row.is_some() {
            return err!("用户已经注册过了");
        }

        // todo: 插入用户数据
        //let mut transaction = self.db.begin().await?;

        let password = password_hash(&payload.password)?;

        let result = sqlx::query!(
            r#"INSERT INTO user (username, password) VALUES(?,?)"#,
            username,
            password
        )
        .execute(self.db)
        .await?;

        //transaction.commit().await?;
        let uid = result.last_insert_rowid();
        let access_token = encode_token(uid, 3600 * 24 * 7)?;
        Ok(UserToken {
            token: access_token.0,
        })
    }
    // 用户密码修改
    pub async fn password_reset(
        &self,
        uid: i64,
        payload: PasswordForm,
    ) -> ItdResult<Json<RespMsg>> {
        // find user by id
        let user_row = sqlx::query_as!(
            UserPasswordRow,
            r#"SELECT password,username FROM user WHERE id = ?"#,
            uid
        )
        .fetch_one(self.db)
        .await?;

        if !verify_password(&payload.old_password, &user_row.password) {
            return err!("旧密码不正确");
        }
        let newpassword = password_hash(&payload.new_password)?;
        sqlx::query!(
            r#"UPDATE user SET password = ? WHERE id = ?"#,
            newpassword,
            uid
        )
        .execute(self.db)
        .await?;
        Ok(Json(RespMsg {
            code: Some(1000),
            message: "密码修改成功".to_string(),
        }))
    }
}
#[derive(Deserialize, Debug, Validate)]
pub struct SignupForm {
    #[validate(regex(path = *USERNAME_REGEX, message = "请填写用户名" ))]
    pub username: String,
    #[validate(custom(function = "crate::utils::validate_password"))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub repassword: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginForm {
    #[validate(regex(path = *USERNAME_REGEX, message = "请填写正确的用户名" ))]
    pub username: String,
    #[validate(custom(function = "crate::utils::validate_password"))]
    pub password: String,
}
#[derive(Deserialize, Debug, Clone, Validate)]
pub struct PasswordForm {
    #[validate(length(min = 8, max = 20, message = "请填写旧密码"))]
    pub old_password: String,
    #[validate(custom(function = "crate::utils::validate_password", message = "密码格式错误"))]
    pub new_password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_insert_admin () -> ItdResult<()> {
         let db = crate::get_conn().await?;
         let user_model = UserModel { db: &db };
         let data = SignupForm {
             username: "admin".to_string(),
             password: "Abc@1234".to_string(),
             repassword: "Abc@1234".to_string(),
         };
         let token = user_model.create_user(data).await?;
         assert_eq!(token.token.is_empty(),false);
         Ok(())
    }
}