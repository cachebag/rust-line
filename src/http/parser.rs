// src/http/parser.rs

use crate::{error::RequestParseError, http::Method};
use std::fmt;

pub struct Header {
    pub name: String,
    pub value: String,
}

pub struct Parser {
    pub method: Method,
    pub target: String,
    pub version_major: u8,
    pub version_minor: u8,
    pub headers: Vec<Header>,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            method: Method::Unknown(String::new()),
            target: String::new(),
            version_major: 0,
            version_minor: 0,
            headers: Vec::new(),
        }
    }

    pub fn extract_and_validate_request(
        &mut self,
        request: &str,
    ) -> Result<(Method, String, u8, u8), RequestParseError> {
        if let Some((req_line, _)) = request.split_once("\r\n") {
            let parts: Vec<&str> = req_line.split_whitespace().collect();

            // if we don't get 3 items, reject it
            if parts.len() != 3 {
                return Err(RequestParseError::InvalidHeaderLength(parts.len()));
            }

            let method_str = parts[0];
            let target = parts[1];
            let version_str = parts[2];

            if !self.is_valid_path(target) {
                return Err(RequestParseError::InvalidReqLine);
            }

            // build the version
            match version_str.strip_prefix("HTTP/") {
                Some(v) if v.contains('.') => {
                    let mut nums = v.split('.');
                    if let (Some(major_str), Some(minor_str)) = (nums.next(), nums.next()) {
                        let major: u8 = major_str.parse().map_err(|_| {
                            RequestParseError::InvalidHttpVersion(version_str.to_string())
                        })?;
                        let minor: u8 = minor_str.parse().map_err(|_| {
                            RequestParseError::InvalidHttpVersion(version_str.to_string())
                        })?;

                        // validate the method type
                        let method = Method::from_method_str(method_str);
                        if !method.is_supported() {
                            return Err(RequestParseError::UnsupportedMethod(method.to_string()));
                        }

                        Ok((method, target.to_string(), major, minor))
                    } else {
                        Err(RequestParseError::InvalidHttpVersion(
                            version_str.to_string(),
                        ))
                    }
                }
                _ => Err(RequestParseError::InvalidHttpVersion(
                    version_str.to_string(),
                )),
            }
        } else {
            Err(RequestParseError::InvalidReqLine)
        }
    }

    pub fn is_valid_path(&self, request: &str) -> bool {
        if !request.starts_with('/') {
            return false;
        }

        if request.is_empty() {
            return false;
        }

        let mut chars = request.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_control() {
                return false;
            }

            if c == ' ' {
                return false;
            }

            if c == '%' {
                let next1 = chars.next();
                let next2 = chars.next();
                match (next1, next2) {
                    (Some(c1), Some(c2)) if c1.is_ascii_hexdigit() && c2.is_ascii_hexdigit() => {}
                    _ => return false,
                }
            }
        }

        true
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Parser {
            method,
            target,
            version_major,
            version_minor,
            headers,
        } = self;

        write!(
            f,
            "{method:?} {target} HTTP/{version_major}.{version_minor}\r\n"
        )?;
        for header in headers {
            write!(f, "{}: {}\r\n", header.name, header.value)?;
        }
        write!(f, "\r\n")
    }
}
