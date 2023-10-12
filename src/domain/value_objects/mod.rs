use crate::domain::errors::domain_error::DomainError;

pub mod unique_id;

pub struct ValueObject<T> {
    value: T,
}

pub trait ValueObjectTrait<T> {
    fn new(value: Option<&str>) -> Result<ValueObject<T>, DomainError>;
    fn value(&self) -> &T;
    fn equals(&self, other: &ValueObject<T>) -> bool;
    fn to_string(&self) -> String;
}

