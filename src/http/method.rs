// src/http/method.rs

use core::fmt;

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

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::HEAD => write!(f, "HEAD"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::CONNECT => write!(f, "CONNECT"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::TRACE => write!(f, "TRACE"),
            Method::Unknown(s) => write!(f, "{s}"),
        }
    }
}
