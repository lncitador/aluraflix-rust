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

pub fn as_description(errors: Vec<DomainError>) -> String {
    let mut description = String::from("");

    for (i, error) in errors.iter().enumerate() {
        description.push_str(error.message.as_str());

        for (i, error) in errors.iter().enumerate() {
            if i == 0 {
                if errors.len() == 1 {
                    description = error.to_string();
                } else {
                    description.push_str("[\n");
                    description.push_str("  ");
                    description.push_str(error.to_string().as_str());
                    description.push_str(",\n");
                }
            } else if i == errors.len() - 1 {
                description.push_str("  ");
                description.push_str(error.to_string().as_str());
                description.push_str("\n]");
            } else {
                description.push_str("  ");
                description.push_str(error.to_string().as_str());
                description.push_str(",\n");
            }
        }
    }

    description
}