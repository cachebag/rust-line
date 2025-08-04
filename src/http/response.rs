// src/http/response.rs

use core::str;
use std::fmt;

pub struct Response {
    status_code: u16,
    reason: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {

    pub fn with_status(status_code: u16, reason: &str, body: String) -> Self {
        let headers = vec![
            ("Content-type".to_string(), "text/plain".to_string()),
            ("Content-length".to_string(), body.len().to_string()),
        ];

        Self {
            status_code,
            reason: reason.to_string(),
            headers,
            body,
        }
    }

    pub fn ok(body: String) -> Self {
        Self::with_status(200, "OK", body)
    }

    pub fn not_found() -> Self {
        Self::with_status(404, "Not Found", String::new())    
    }

    pub fn bad_request() -> Self {
        Self::with_status(400, "Bad Request", String::new())
    }

    pub fn internal_error() -> Self {
        Self::with_status(500, "Internal Server Error", String::new())
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        if let Some(existing) = self.headers.iter_mut().find(|(k, _)| k.eq_ignore_ascii_case(key)) {
            existing.1 = value.to_string();
        } else {
            self.headers.push((key.to_string(), value.to_string()));
        }
    }

    pub fn content_type(mut self, value: &str) -> Self {
        self.set_header("Content-Type", value);
        self 
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.set_header(key, value);
        self
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP/1.1 {} {}\r\n", self.status_code, self.reason)?;

        for (name, value) in &self.headers {
            write!(f, "{}: {}\r\n", name, value)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}
