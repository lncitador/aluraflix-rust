use time::{Date, OffsetDateTime};
use serde::{Serialize, Deserialize};
use crate::domain::errors::domain_error::{as_descriptions, DomainError};
use crate::domain::value_objects::color::ColorEntity;
use crate::domain::value_objects::unique_id::UniqueEntityID;
use crate::domain::value_objects::ValueObjectTrait;

#[derive(Deserialize)]
pub struct CategoriesInput {
    pub name: String,
    pub color: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Categories {
    pub id: UniqueEntityID,
    pub name: String,
    pub color: ColorEntity,
    pub user_id: UniqueEntityID,
    pub created_at: Date,
    pub updated_at: Date,
}

impl Categories {
    pub fn new(data: &CategoriesInput) -> Result<Self, DomainError> {
        let mut errors: Vec<DomainError> = vec![];

        let name = match data.name.len() {
            0 => {
                errors.push(DomainError::new("Name is required", ""));
                None
            }
            1 | 2 | 3 => {
                errors.push(DomainError::new("Name must be at least 4 characters", ""));
                None
            }
            _ => Some(data.name.to_string())
        };

        let color = match ColorEntity::new(Some(data.color.as_str())) {
            Ok(color) => Some(color),
            Err(error) => {
                errors.push(error);
                None
            }
        };

        let user_id = match UniqueEntityID::new(Some(data.user_id.as_str())) {
            Ok(user_id) => Some(user_id),
            Err(error) => {
                errors.push(error);
                None
            }
        };

        if errors.len() > 0 {
            let description = as_descriptions(errors);

            return Err(DomainError::new("Invalid data", description.as_str()))
        }


        let now = OffsetDateTime::now_utc().date();

        Ok(Categories {
            id: UniqueEntityID::new(None).unwrap(),
            name: name.unwrap(),
            color: color.unwrap(),
            user_id: user_id.unwrap(),
            created_at: now,
            updated_at: now,
        })
    }
}