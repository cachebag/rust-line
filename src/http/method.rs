// src/http/method.rs

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    Unknown(String),
}

impl Method {
    pub fn from_method_str(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "CONNECT" => Method::CONNECT,
            "OPTIONS" => Method::OPTIONS,
            "TRACE" => Method::TRACE,
            other => Method::Unknown(other.to_string()),
        }
    }

    pub fn is_supported(&self) -> bool {
        !matches!(self, Method::Unknown(_))
    }
}
