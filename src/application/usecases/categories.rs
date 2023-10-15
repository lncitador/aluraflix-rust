use std::fmt::{Debug, Formatter};
use crate::application::repositories::categories::CategoriesRepositoryContract;
use crate::application::repositories::RepositoryError;
use crate::domain::errors::app_error::AppError;
use crate::domain::errors::domain_error::DomainError;

pub struct CategoriesUseCase {
    categories_repository: CategoriesRepositoryContract,
}

pub enum CategoriesUseCaseError {
    CategoriesNotFound,
    Domain(DomainError),
}

impl From<CategoriesUseCaseError> for AppError {
    fn from(error: CategoriesUseCaseError) -> Self {
        match error {
            CategoriesUseCaseError::CategoriesNotFound => AppError::new("Categories not found", 404, None),
            CategoriesUseCaseError::Domain(domain) => AppError::new("Categories domain error", 442, Some(domain))
        }
    }
}

impl From<RepositoryError> for CategoriesUseCaseError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::NotFound(_) => CategoriesUseCaseError::CategoriesNotFound,
            RepositoryError::Domain(error) => CategoriesUseCaseError::Domain(error),
            RepositoryError::AlreadyExists(_) => CategoriesUseCaseError::Domain(DomainError::new("Category already exists", "")),
        }
    }
}

impl Debug for CategoriesUseCaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoriesUseCaseError::CategoriesNotFound => write!(f, "Categories not found"),
            CategoriesUseCaseError::Domain(error) => write!(f, "{:?}", error),
        }
    }
}

impl CategoriesUseCase {
    pub fn new(categories_repository: CategoriesRepositoryContract) -> Self {
        Self {
            categories_repository,
        }
    }
}