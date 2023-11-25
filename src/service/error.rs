#![allow(clippy::enum_variant_names)]
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;
use validator::ValidationErrors;

use crate::{model::ErrorResponse, repository::DbError};

pub type Result<T> = core::result::Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    RepositoryError(#[from] DbError),
    #[error("Validation errors: {0}")]
    ValidationError(#[from] ValidationErrors),
    #[error("Internal error: {0}")]
    InternalError(#[from] Box<dyn std::error::Error>),
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::RepositoryError(DbError::NotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::ValidationError(ref e) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    message: "Validation failed".to_string(),
                    error: format!("{:?}", e),
                })
            }
            ServiceError::RepositoryError(DbError::NotFound) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    message: "Not Found".to_string(),
                    error: "Not Found".to_string(),
                })
            }
            _ => HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
                error: format!("{:?}", self),
            }),
        }
    }
}
