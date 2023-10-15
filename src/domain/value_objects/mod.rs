use crate::domain::errors::domain_error::DomainError;

mod __tests__;

pub mod unique_id;
pub mod url;
pub mod color;
pub mod email;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ValueObject<T> {
    value: T,
}

pub trait ValueObjectTrait<T> {
    fn new(value: Option<&str>) -> Result<ValueObject<T>, DomainError>;
    fn value(&self) -> &T;
    fn equals(&self, other: &ValueObject<T>) -> bool;
    fn to_string(&self) -> String;
}
