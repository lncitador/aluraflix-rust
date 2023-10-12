use crate::domain::value_objects::unique_id::UniqueEntityID;
use crate::domain::value_objects::url::UrlEntity;
use time::Date;

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