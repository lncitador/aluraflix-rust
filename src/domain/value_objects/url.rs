use std::fmt::{Debug, Display, Formatter};
use lazy_static::lazy_static;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObject, ValueObjectTrait};
use regex::Regex;

lazy_static! {
    pub static ref URL_REGEX: Regex = Regex::new(
        r"https?://(?:www\.)?([-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b)*(/[/\d\w\.-]*)*(?:[\?])*(.+)*"
    ).unwrap();
}

pub type UrlEntity = ValueObject<String>;

impl Debug for UrlEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

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
                if URL_REGEX.is_match(value) {
                    let groups = URL_REGEX.captures(value).unwrap();
                    let path = groups.get(1);

                    if let Some(path) = path {
                        if !path.as_str().contains(".com") {
                            return Err(DomainError::new("Can't commercial URL", ""))
                        }
                    } else {
                        return Err(DomainError::new("Invalid URL", ""))
                    }

                    Ok(UrlEntity {
                        value: value.to_string()
                    })
                } else {
                    Err(DomainError::new("Invalid URL", ""))
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

