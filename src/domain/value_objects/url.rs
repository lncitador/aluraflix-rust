use std::str::FromStr;
use http::Uri;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObject, ValueObjectTrait};

pub type UrlEntity = ValueObject<Uri>;

impl ValueObjectTrait<Uri> for UrlEntity {
    fn new(value: Option<&str>) -> Result<ValueObject<Uri>, DomainError> {
        match value {
            Some(value) => {
                match Uri::from_str(value) {
                    Ok(value) => Ok(ValueObject { value }),
                    Err(_) => Err(DomainError::new("Invalid URL", ""))
                }
            },
            None => Err(DomainError::new("URL is required", ""))
        }
    }

    fn value(&self) -> &Uri {
        &self.value
    }

    fn equals(&self, other: &ValueObject<Uri>) -> bool {
        &self.value == other.value()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

