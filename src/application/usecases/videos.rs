use std::fmt::{Debug, Formatter};
use crate::application::repositories::RepositoryError;
use crate::application::repositories::videos::VideosRepositoryContract;
use crate::domain::errors::app_error::AppError;
use crate::domain::errors::domain_error::DomainError;

pub struct VideosUseCase {
    videos_repository: VideosRepositoryContract,
}

pub enum VideosUseCaseError {
    VideosNotFound,
    Domain(DomainError),
}

impl From<VideosUseCaseError> for AppError {
    fn from(error: VideosUseCaseError) -> Self {
        match error {
            VideosUseCaseError::VideosNotFound => AppError::new("Videos not found", 404, None),
            VideosUseCaseError::Domain(domain) => AppError::new("Videos domain error", 442, Some(domain))
        }
    }
}

impl From<RepositoryError> for VideosUseCaseError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::NotFound(_) => VideosUseCaseError::VideosNotFound,
            RepositoryError::Domain(error) => VideosUseCaseError::Domain(error),
            RepositoryError::AlreadyExists(_) => VideosUseCaseError::Domain(DomainError::new("Video already exists","")),
        }
    }
}

impl Debug for VideosUseCaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VideosUseCaseError::VideosNotFound => write!(f, "Videos not found"),
            VideosUseCaseError::Domain(error) => write!(f, "{:?}", error),
        }
    }
}

impl VideosUseCase {
    pub fn new(videos_repository: VideosRepositoryContract) -> Self {
        Self {
            videos_repository,
        }
    }
}