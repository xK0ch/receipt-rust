use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use log::error;
use serde_json::json;
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, ToSchema)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        ApiError {
            status_code,
            message,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
            DieselError::NotFound => ApiError::new(404, "Record not found".to_string()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl From<(DieselError, String)> for ApiError {
    fn from((error, message): (DieselError, String)) -> ApiError {
        match error {
            DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
            DieselError::NotFound => ApiError::new(404, message.to_string()),
            err => ApiError::new(500, format!("Diesel error: {}", err)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = StatusCode::from_u16(self.status_code)
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => {
                error!("{}", self.message);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status_code)
            .json(json!({ "status": status_code.as_u16(), "message": message }))
    }
}

pub fn not_found<T>(error: DieselError, entity_name: &str, entity_id: Uuid) -> Result<T, ApiError> {
    match error {
        DieselError::NotFound => Err(ApiError::from((
            error,
            format!("{} with id: {} not found", entity_name, entity_id),
        ))
        .into()),
        _ => Err(error.into()),
    }
}
