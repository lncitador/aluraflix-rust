use uuid::Uuid;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObjectTrait, ValueObject};

pub type UniqueEntityID = ValueObject<Uuid>;

impl ValueObjectTrait<Uuid> for UniqueEntityID {
     fn new(value: &str) -> Result<ValueObject<Uuid>, DomainError> {
        if value.is_empty() {
            Ok(ValueObject { value: Uuid::now_v7() })
        } else {
            match Uuid::parse_str(value) {
                Ok(uuid) => Ok(ValueObject { value: uuid }),
                Err(_) => Err(DomainError::new("The ID is not a valid UUID", ""))
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