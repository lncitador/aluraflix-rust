use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct DomainError {
    pub message: String,
    pub description: Option<String>
}

impl DomainError {
    pub fn new(message: &str, description: &str) -> DomainError {
        DomainError {
            message: message.to_string(),
            description: if description.is_empty() { None } else { Some(description.to_string()) }
        }
    }
}

impl Error for DomainError {}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.description {
            Some(ref description) => write!(f, "Domain Error: {}\nDescription: {}", self.message, description),
            None => write!(f, "Domain Error: {}", self.message)
        }
    }
}

impl Debug for DomainError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.description {
            Some(ref description) => write!(f, "Domain Error: {}\nDescription: {}", self.message, description),
            None => write!(f, "Domain Error: {}", self.message)
        }
    }
}