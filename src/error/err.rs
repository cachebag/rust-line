use std::fmt;

#[derive(Debug)]
pub enum RequestParseError {
    EmptyReqLine,
    MissingMethod,
    MissingPath,
    MissingHttpVersion,
    UnsupportedMethod(String),
    InvalidHttpVersion(String),
}

impl fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestParseError::EmptyReqLine => {
                write!(f, "Missing request.")
            }
            RequestParseError::MissingMethod => {
                write!(f, "Missing method.")
            }
            RequestParseError::MissingPath => {
                write!(f, "Missing path.")
            }
            RequestParseError::MissingHttpVersion => {
                write!(f, "Missing HTTP version.")
            }
            RequestParseError::UnsupportedMethod(found) => {
                write!(f, "{found} is not a supported method.")
            }
            RequestParseError::InvalidHttpVersion(found) => {
                write!(f, "{found} is not currently supported.")
            }
        }
    }
}
