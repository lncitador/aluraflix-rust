use std::fmt::{Debug, Formatter};
use crate::application::repositories::RepositoryError;
use crate::application::repositories::users::{UsersRepository, UsersRepositoryContract};
use crate::domain::entities::users::{Users, UsersInput};
use crate::domain::errors::app_error::AppError;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::email::EmailEntity;
use crate::domain::value_objects::ValueObjectTrait;

pub struct AuthUseCase {
    pub users_repository: UsersRepositoryContract
}

pub struct SignInInput {
    pub email: String,
    pub password: String,
}

impl AuthUseCase {
    pub fn new(users_repository: UsersRepositoryContract) -> Self {
        Self {
            users_repository,
        }
    }

    pub fn sign_in(&self, input: SignInInput) -> Result<String, AuthUseCaseError> {
        let email = match EmailEntity::new(Some(&input.email)) {
            Ok(email) => email,
            Err(_) => return Err(AuthUseCaseError::UserNotFound),
        };

        let mut users_repository = self.users_repository.clone();

        let user = match users_repository.lock() {
            Ok(repo) => {
                match repo.find_by_email(email) {
                    Some(user) => user,
                    None => return Err(AuthUseCaseError::UserNotFound),
                }
            }
            Err(err) => {
                return Err(AuthUseCaseError::Domain(DomainError::new("Repository lock error", err.to_string().as_str())));
            }
        };

        // if !user.password.is_valid(&input.password) {
        //    return Err(AuthUseCaseError::InvalidPassword);
        // }

        if user.password != input.password {
            return Err(AuthUseCaseError::InvalidPassword);
        }

        Ok("User logged in".to_string())
    }

    pub fn sign_up(&mut self, input: UsersInput) -> Result<Users, AuthUseCaseError> {
        let email = match EmailEntity::new(Some(&input.email)) {
            Ok(email) => email,
            Err(error) => return Err(AuthUseCaseError::Domain(error)),
        };


        let user_already_exists = match self.users_repository.lock() {
            Ok(repo) => {
                match repo.find_by_email(email) {
                    Some(_) => true,
                    None => false,
                }
            }
            Err(err) => {
                return Err(AuthUseCaseError::Domain(DomainError::new("Repository lock error", err.to_string().as_str())));
            }
        };

        if user_already_exists {
            return Err(AuthUseCaseError::UserAlreadyExists);
        }

        let user = match Users::new(&input) {
            Ok(user) => user,
            Err(error) => {
                return Err(AuthUseCaseError::Domain(error));
            }
        };

        match self.users_repository.lock() {
            Ok(mut repo) => {
                match repo.save(user) {
                    Ok(user) => Ok(user),
                    Err(error) => Err(AuthUseCaseError::from(error)),
                }
            }
            Err(err) => {
                Err(AuthUseCaseError::Domain(DomainError::new("Repository lock error", err.to_string().as_str())))
            }
        }
    }
}
