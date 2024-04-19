use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BasketErrors {
    #[error("Basket not found.")]
    BasketNotFound,
    #[error("Basket is expired.")]
    BasketExpired,
    #[error("Basket is unpaid.")]
    BasketUnpaid,
    #[error("{0}")]
    Unknown(#[from] anyhow::Error)
}

pub struct BasketError {
    pub error: BasketErrors,
}


// Tell axum how to convert `BasketErrors` into a response.
impl IntoResponse for BasketError {
    fn into_response(self) -> Response {
        match self.error {
            BasketErrors::BasketNotFound => (StatusCode::BAD_REQUEST, format!("{}", BasketErrors::BasketNotFound)),
            BasketErrors::BasketExpired => (StatusCode::BAD_REQUEST, format!("{}", BasketErrors::BasketExpired)),
            BasketErrors::BasketUnpaid => (StatusCode::BAD_REQUEST, format!("{}", BasketErrors::BasketUnpaid)),
            BasketErrors::Unknown(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)),
        }.into_response()
    }
}

impl From<anyhow::Error> for BasketError
{
    fn from(err: anyhow::Error) -> Self {
        match err.downcast::<BasketErrors>() {
            Ok(err) => 
            Self {
                error: err,
            },
            Err(err) => 
            Self {
                error: BasketErrors::Unknown(err).into(),
            },
        }
    }
}
