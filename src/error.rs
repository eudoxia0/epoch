use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;

/// Represents an error in the application.
#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl AppError {
    /// Create a new error from a message.
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

/// A shorthand error type.
pub type Fallible<T> = Result<T, AppError>;

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::new(format!("I/O error: {value}"))
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(value: rusqlite::Error) -> Self {
        AppError::new(format!("rusqlite: {value}"))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
    }
}
