use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use validator::Validate;
// use crate::AppState;
use crate::error::ItdError as Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedData<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedData<T>
where
    T: DeserializeOwned + Validate + Debug,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    //AppState: FromRef<S>,
    //B: Send + 'static,
{
    type Rejection = Error;
    //type Rejection = (StatusCode, axum::Json<Value>);
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = axum::Json::<T>::from_request(req, state).await?;
        // println!(" value is {:?}",value);
        //let app_state = AppState::from_ref(state);

        value.validate()?;
        Ok(Self(value))
    }
}
