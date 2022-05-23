use std::{io, fmt};
use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
enum ErrorType {
    DbError,
    Reqwest,
    Internal,
    Unknown,
}


#[derive(Debug)]
struct ServerError {
    pub error_message: String,
    pub error_type: ErrorType,
}

#[derive(Serialize)]
pub struct ErrResponse {
    pub error: String,
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
        ServerError {
            error_message: format!("{error}"),
            error_type: ErrorType::Unknown,
        }
    }
}

impl From<anyhow::Error> for ServerError {
    fn from(error: anyhow::Error) -> Self {
        ServerError {
            error_message: format!("{error}"),
            error_type: ErrorType::Unknown,
        }
    }
}

impl From<reqwest::Error> for ServerError {
    fn from(error: reqwest::Error) -> Self {
        ServerError {
            error_message: format!("{error}"),
            error_type: ErrorType::Reqwest,
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error -> {self:?}")
    }
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            ErrorType::DbError => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorType::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::Reqwest => StatusCode::SERVICE_UNAVAILABLE,
            ErrorType::Unknown => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrResponse{error: self.error_message.clone()})
    }
}