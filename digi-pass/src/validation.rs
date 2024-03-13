use async_trait::async_trait;
use axum::{extract::{rejection::JsonRejection, FromRequest, Request}, http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::de::DeserializeOwned;
use validator::Validate;
use thiserror::Error;


#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(errors) => {
                //let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, Json(errors.into_errors())).into_response()
            }
            ServerError::AxumJsonRejection(rejection) => (StatusCode::BAD_REQUEST, rejection).into_response(),
        }
        .into_response()
    }
}