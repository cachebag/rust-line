// src/http/status_codes.rs

use crate::error::RequestParseError;

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    MethodNotAllowed = 405,
    UriTooLong = 414,
    InternalServerError = 500,
    NotImplemented = 501,
    HttpVersionNotSupported = 505,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::UriTooLong => "URI Too Long",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
        }
    }
}

impl From<RequestParseError> for StatusCode {
    fn from(error: RequestParseError) -> Self {
        match error {
            RequestParseError::UnsupportedMethod(_) => StatusCode::MethodNotAllowed,
            RequestParseError::InvalidHttpVersion(_) => StatusCode::HttpVersionNotSupported,
            RequestParseError::InvalidFieldLength(_) => StatusCode::UriTooLong,
            _ => StatusCode::BadRequest,
        }
    }
}
