// src/http/response.rs

use std::fmt;

pub struct Response {
    status_code: u16,
    reason: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    pub fn ok(body: String) -> Self {
        Self {
            status_code: 200,
            reason: "OK".to_string(),
            headers: vec![
                ("Content-Type".to_string(), "text/plain".to_string()),
                ("Content-Length".to_string(), body.len().to_string()),
            ],
            body,
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            reason: "Not Found".to_string(),
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn bad_request() -> Self {
        Self {
            status_code: 400,
            reason: "Bad Request".to_string(),
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            status_code: 500,
            reason: "Internal Server Error".to_string(),
            headers: Vec::new(),
            body: String::new(),
        }
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
