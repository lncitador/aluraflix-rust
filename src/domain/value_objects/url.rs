use std::str::FromStr;
use http::Uri;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObject, ValueObjectTrait};

pub type UrlEntity = ValueObject<String>;

impl TryFrom<UrlEntity> for String {
    type Error = DomainError;

impl ValueObjectTrait<Uri> for UrlEntity {
    fn new(value: Option<&str>) -> Result<ValueObject<Uri>, DomainError> {
        match value {
            Some(value) => {
                match Uri::from_str(value) {
                    Ok(value) => Ok(ValueObject { value: value.to_string() }),
                    Err(_) => Err(DomainError::new("Invalid URL", ""))
                }
            },
            None => Err(DomainError::new("URL is required", ""))
        }
    }

    fn value(&self) -> &String {
        &self.value
    }

    fn equals(&self, other: &UrlEntity) -> bool {
        &self.value == other.value()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

