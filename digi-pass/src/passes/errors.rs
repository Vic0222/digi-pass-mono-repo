use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PassErrors {
    #[error("{0}")]
    Unknown(#[from] anyhow::Error)
}

pub struct PassError {
    pub error: PassErrors,
}


// Tell axum how to convert `PassErrors` into a response.
impl IntoResponse for PassError {
    fn into_response(self) -> Response {
        match self.error {
            PassErrors::Unknown(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)),
        }.into_response()
    }
}

impl From<anyhow::Error> for PassError
{
    fn from(err: anyhow::Error) -> Self {
        match err.downcast::<PassErrors>() {
            Ok(err) => 
            Self {
                error: err,
            },
            Err(err) => 
            Self {
                error: PassErrors::Unknown(err).into(),
            },
        }
    }
}
