use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use crate::domain::errors::domain_error::{as_descriptions, DomainError};
use crate::domain::value_objects::email::EmailEntity;
use crate::domain::value_objects::unique_id::UniqueEntityID;
use crate::domain::value_objects::ValueObjectTrait;

#[derive(Deserialize)]
pub struct UsersInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Users {
    pub id: UniqueEntityID,
    pub name: String,
    pub email: EmailEntity,
    pub password: String,
    pub created_at: Date,
    pub updated_at: Date,
}

impl Users {
    pub fn new(data: &UsersInput) -> Result<Self, DomainError> {
        let id = UniqueEntityID::new(None).unwrap();

        let now = OffsetDateTime::now_utc().date();

        let mut errors: Vec<DomainError> = vec![];

        let name = match data.name.len() {
            0 => {
                errors.push(DomainError::new("Name is required", ""));
                None
            },
            1 | 2 | 3 => {
                errors.push(DomainError::new("Name must be at least 4 characters", ""));
                None
            },
            _ => Some(data.name.to_string())
        };

        let email = match EmailEntity::new(Some(data.email.as_str())) {
            Ok(email) => Some(email),
            Err(error) => {
                errors.push(error);
                None
            }
        };

        let password = match data.password.len() {
            0 => {
                errors.push(DomainError::new("Password is required", ""));
                None
            },
            1 | 2 | 3 | 4 | 5 | 6 | 7  => {
                errors.push(DomainError::new("Password must be at least 8 characters", ""));
                None
            },
            _ => Some(data.password.to_string())
        };

        if errors.len() > 0 {
            let description = as_descriptions(errors);

            return Err(DomainError::new("Invalid data", description.as_str()))
        }

        Ok(Users {
            id,
            name: name.unwrap(),
            email: email.unwrap(),
            password: password.unwrap(),
            created_at: now,
            updated_at: now,
        })
    }
}