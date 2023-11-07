use std::fmt::Debug;
use async_trait::async_trait;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::unique_id::UniqueEntityID;

pub mod videos;
pub mod categories;
pub mod users;

pub enum RepositoryError {
    NotFound(String),
    AlreadyExists(String),
    Domain(DomainError),
}

impl From<RepositoryError> for DomainError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::NotFound(message) => DomainError::new("Not found", &message),
            RepositoryError::AlreadyExists(message) => DomainError::new("Already exists", &message),
            RepositoryError::Domain(error) => error,
        }
    }
}

impl Debug for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound(message) => write!(f, "Not found: {}", message),
            RepositoryError::AlreadyExists(message) => write!(f, "Already exists: {}", message),
            RepositoryError::Domain(error) => write!(f, "{:?}", error),
        }
    }
}

#[async_trait]
pub trait Repository<T> {
    async fn find_all(&self) -> Vec<T>;
    async fn find_by_id(&self, id: UniqueEntityID) -> Result<T, RepositoryError>;
    async fn save(&mut self, entity: T) -> Result<T, RepositoryError>;
    async fn delete(&mut self, id: UniqueEntityID) -> Option<RepositoryError>;
}