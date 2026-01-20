use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub struct ErrorBody<'a, T: Serialize> {
    pub code: &'a str,
    pub message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<&'a T>,
}

pub struct ApiError<T: Serialize> {
    pub status: u16,
    pub code: &'static str,
    pub message: &'static str,
    pub details: Option<T>,
}

impl<T: Serialize> fmt::Debug for ApiError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
            .field("status", &self.status)
            .field("code", &self.code)
            .field("message", &self.message)
            .finish()
    }
}

impl<T: Serialize> fmt::Display for ApiError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl<T: Serialize> ApiError<T> {
    pub fn bad_request(code: &'static str, message: &'static str, details: Option<T>) -> Self {
        Self {
            status: 400,
            code,
            message,
            details,
        }
    }

    pub fn unauthorized(code: &'static str, message: &'static str) -> Self {
        Self {
            status: 401,
            code,
            message,
            details: None,
        }
    }

    pub fn forbidden(code: &'static str, message: &'static str) -> Self {
        Self {
            status: 403,
            code,
            message,
            details: None,
        }
    }

    pub fn not_found(code: &'static str, message: &'static str) -> Self {
        Self {
            status: 404,
            code,
            message,
            details: None,
        }
    }

    pub fn conflict(code: &'static str, message: &'static str) -> Self {
        Self {
            status: 409,
            code,
            message,
            details: None,
        }
    }

    pub fn unprocessable(code: &'static str, message: &'static str, details: Option<T>) -> Self {
        Self {
            status: 422,
            code,
            message,
            details,
        }
    }

    pub fn service_unavailable(code: &'static str, message: &'static str) -> Self {
        Self {
            status: 503,
            code,
            message,
            details: None,
        }
    }

    pub fn internal() -> Self {
        Self {
            status: 500,
            code: "internal_error",
            message: "Internal server error",
            details: None,
        }
    }
}

impl<T: Serialize> ResponseError for ApiError<T> {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        let body = ErrorBody {
            code: self.code,
            message: self.message,
            details: self.details.as_ref(),
        };
        let json = serde_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(json)
    }
}

// ApiError is translated into HTTP responses by Actix Web.
