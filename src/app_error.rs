use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use serde::Serialize;

#[derive(Serialize, Debug)]
struct ErrorResponse {
    status_code: u16,
    error: String,
}

impl ErrorResponse {
    pub fn new(status_code: u16, message: String) -> Self {
        Self {
            status_code,
            error: message,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("DatabaseError: {0}")]
    DatabaseError(surrealdb::Error),

    #[allow(dead_code)]
    #[error("{0}")]
    BadRequest(String),

    #[error("IOError: {0}")]
    IOError(std::io::Error),

    #[allow(dead_code)]
    #[error("InternalServerError: {0}")]
    InternalError(String),

    #[allow(dead_code)]
    #[error("UNAUTHORIZED please login first..")]
    UnAuthorized,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code().as_u16();
        let error_message = self.to_string();
        let error_response = ErrorResponse::new(status_code, error_message);

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(error_response)
    }
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::UnAuthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(value: surrealdb::Error) -> Self {
        AppError::DatabaseError(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::IOError(value)
    }
}
