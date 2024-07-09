use reqwest::StatusCode;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct VndbApiError {
    pub status: Option<StatusCode>,
    pub message: String,
}

impl Error for VndbApiError {}

impl fmt::Display for VndbApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error ({:#?}): {:#?}", self.status, self.message)
    }
}

impl VndbApiError {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self {
            status: Some(status),
            message,
        }
    }
}

impl From<reqwest::Error> for VndbApiError {
    fn from(err: reqwest::Error) -> Self {
        let status = err.status();
        let message = format!("Reqwest error: {}", err);

        Self { status, message }
    }
}
