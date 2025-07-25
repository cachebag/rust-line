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
            StatusCode::Ok => "OK\n",
            StatusCode::BadRequest => "Bad Request\n",
            StatusCode::NotFound => "Not Found\n",
            StatusCode::MethodNotAllowed => "Method Not Allowed\n",
            StatusCode::UriTooLong => "URI Too Long\n",
            StatusCode::InternalServerError => "Internal Server Error\n",
            StatusCode::NotImplemented => "Not Implemented\n",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported\n",
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
