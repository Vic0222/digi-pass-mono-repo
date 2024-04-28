use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderErrors {
    #[error("{0}")]
    Unknown(#[from] anyhow::Error)
}

pub struct OrderError {
    pub error: OrderErrors,
}


// Tell axum how to convert `OrderErrors` into a response.
impl IntoResponse for OrderError {
    fn into_response(self) -> Response {
        let status_code = match &self.error {
            OrderErrors::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, format!("{}", self.error)).into_response()
    }
}

impl From<anyhow::Error> for OrderError
{
    fn from(err: anyhow::Error) -> Self {
        match err.downcast::<OrderErrors>() {
            Ok(err) => 
            Self {
                error: err,
            },
            Err(err) => 
            Self {
                error: OrderErrors::Unknown(err).into(),
            },
        }
    }
}
