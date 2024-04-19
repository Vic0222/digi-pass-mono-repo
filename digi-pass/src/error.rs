//The over arching error type. Will be slowly replaced with feature speicific erros
use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub struct AppError{
    pub error: anyhow::Error,
    pub status: StatusCode,
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        (
            self.status,
            format!("Something went wrong: {}", self.error),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        
        Self {
            error: err.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

