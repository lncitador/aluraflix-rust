use std::str::FromStr;
use http::Uri;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObject, ValueObjectTrait};

pub type UrlEntity = ValueObject<String>;

impl TryFrom<UrlEntity> for String {
    type Error = DomainError;

    fn try_from(value: UrlEntity) -> Result<Self, Self::Error> {
        Ok(value.value.to_string())
    }
}

impl From<String> for UrlEntity {
    fn from(value: String) -> Self {
        UrlEntity::new(Some(value.as_str())).unwrap()
    }
}

impl ValueObjectTrait<String> for UrlEntity {
    fn new(value: Option<&str>) -> Result<UrlEntity, DomainError> {
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

