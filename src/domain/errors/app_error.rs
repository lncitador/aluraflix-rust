use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct AppError {
    message: String,
    code: u32,
}

impl AppError {
    pub fn new(message: &str, code: u32) -> AppError {
        AppError {
            message: message.to_string(),
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
