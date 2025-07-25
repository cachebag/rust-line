use std::fmt;

#[derive(Debug)]
pub enum RequestParseError {
    InvalidHeaderLength(usize),
    InvalidReqLine,
    MissingMethod,
    MissingPath,
    MissingHttpVersion,
    UnsupportedMethod(String),
    InvalidHttpVersion(String),
}

impl fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestParseError::InvalidHeaderLength(length) => {
                write!(
                    f,
                    "{} does not match the required length of this header.",
                    length
                )
            }
            RequestParseError::InvalidReqLine => {
                write!(f, "Invalid request.")
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
