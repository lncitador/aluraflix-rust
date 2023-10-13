use crate::domain::value_objects::unique_id::UniqueEntityID;
use crate::domain::value_objects::url::UrlEntity;
use crate::domain::errors::domain_error::DomainError;

use time::{Date, OffsetDateTime};
use serde::{Serialize, Deserialize};
use crate::domain::value_objects::ValueObjectTrait;

#[derive(Deserialize)]
pub struct VideosInput {
    pub title: String,
    pub description: String,
    pub url: String,
    pub category_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Videos {
    pub id: UniqueEntityID,
    pub title: String,
    pub description: String,
    pub url: UrlEntity,
    pub category_id: UniqueEntityID,
    pub user_id: UniqueEntityID,
    pub created_at: Date,
    pub updated_at: Date,
}

impl Videos {
    pub fn new(data: &VideosInput) -> Result<Self, DomainError> {
        let mut errors: Vec<DomainError> = vec![];

        let title = match data.title.len() {
            0 => {
                errors.push(DomainError::new("Title is required", ""));
                None
            },
            1 | 2 | 3 => {
                errors.push(DomainError::new("Title must be at least 4 characters", ""));
                None
            },
            _ => Some(data.title.to_string())
        };

        let description = match data.description.len() {
            0 => {
                errors.push(DomainError::new("Description is required", ""));
                None
            },
            1 | 2 | 3 | 4 => {
                errors.push(DomainError::new("Description must be at least 5 characters", ""));
                None
            },
            _ => Some(data.description.to_string())
        };

        let url = match UrlEntity::new(Some(data.url.as_str())) {
            Ok(url) => Some(url),
            Err(error) => {
                errors.push(error);
                None
            }
        };

        let category_id = match UniqueEntityID::new(Some(data.category_id.as_str())) {
            Ok(category_id) => Some(category_id),
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
            let mut description = String::from("");

            for (i, error) in errors.iter().enumerate() {
                if i == 0 {
                    if errors.len() == 1 {
                        description = error.to_string();
                    } else {
                        description.push_str("[\n");
                        description.push_str("  ");
                        description.push_str(error.to_string().as_str());
                        description.push_str(",\n");
                    }
                } else if i == errors.len() - 1 {
                    description.push_str("  ");
                    description.push_str(error.to_string().as_str());
                    description.push_str("\n]");
                } else {
                    description.push_str("  ");
                    description.push_str(error.to_string().as_str());
                    description.push_str(",\n");
                }
            }

            return Err(DomainError::new("Invalid data", description.as_str()));
        }

        let now = OffsetDateTime::now_utc().date();

        Ok(Videos {
            id: UniqueEntityID::new(None).unwrap(),
            title: title.unwrap(),
            description: description.unwrap(),
            url: url.unwrap(),
            category_id: category_id.unwrap(),
            user_id: user_id.unwrap(),
            created_at: now,
            updated_at: now,
        })
    }
}
