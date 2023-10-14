use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use crate::domain::errors::domain_error::DomainError;

pub struct AppError {
    message: String,
    domain: Option<DomainError>,
    code: u32,
}

impl AppError {
    pub fn new(message: &str, code: u32, domain: Option<DomainError>) -> AppError {
        AppError {
            message: message.to_string(),
            domain,
            code,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn code(&self) -> u32 {
        self.code
    }
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Application Error: {}, code: {}", self.message, self.code)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Application Error: {}, code: {}", self.message, self.code)
    }
}
