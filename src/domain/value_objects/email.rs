use std::fmt::{Debug, Display, Formatter};
use regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObject, ValueObjectTrait};

lazy_static! {
    pub static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"
    ).unwrap();
}

#[derive(Serialize, Deserialize)]
pub enum Email {
    Email(String)
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Email::Email(value) => Display::fmt(value, f)
        }
    }
}

pub type EmailEntity = ValueObject<Email>;

impl Debug for EmailEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl TryFrom<EmailEntity> for String {
    type Error = DomainError;

    fn try_from(value: EmailEntity) -> Result<Self, Self::Error> {
        Ok(value.value.to_string())
    }
}

impl From<String> for EmailEntity {
    fn from(value: String) -> Self {
        EmailEntity::new(Some(value.as_str())).unwrap()
    }
}

impl ValueObjectTrait<Email> for EmailEntity {
    fn new(value: Option<&str>) -> Result<EmailEntity, DomainError> {
        match value {
            Some(value) => {
                if EMAIL_REGEX.is_match(value) {
                    Ok(EmailEntity {
                        value: Email::Email(value.to_string())
                    })
                } else {
                    Err(DomainError::new("Invalid email", ""))
                }
            },
            None => Err(DomainError::new("Email is required", ""))
        }
    }

    fn value(&self) -> &Email {
        &self.value
    }

    fn equals(&self, other: &EmailEntity) -> bool {
        self.value.to_string() == other.value.to_string()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}