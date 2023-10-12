use uuid::Uuid;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObjectTrait, ValueObject};

pub type UniqueEntityID = ValueObject<Uuid>;

impl ValueObjectTrait<Uuid> for UniqueEntityID {
     fn new(value: Option<&str>) -> Result<ValueObject<Uuid>, DomainError> {
        match value {
            Some(value) => {
                match Uuid::parse_str(value) {
                    Ok(value) => Ok(ValueObject { value }),
                    Err(_) => Err(DomainError::new("Invalid UUID", ""))
                }
            },
            None => {
                let value = Uuid::now_v7();
                Ok(ValueObject { value })
            }
        }
    }

    fn value(&self) -> &Uuid {
        &self.value
    }

    fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}