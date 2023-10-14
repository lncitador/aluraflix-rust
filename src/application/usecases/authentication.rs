use std::rc::Rc;
use crate::application::repositories::users::UsersRepository;
use crate::domain::errors::app_error::AppError;
use crate::domain::value_objects::email::EmailEntity;
use crate::domain::value_objects::ValueObjectTrait;

pub struct AuthUseCase {
    pub users_repository: Rc<dyn UsersRepository>,
}

pub struct SignInInput {
    pub email: String,
    pub password: String,
}

impl AuthUseCase {
    pub fn new(users_repository: Rc<dyn UsersRepository>) -> Self {
        Self {
            users_repository,
        }
    }

    pub fn sign_in(&self, input: SignInInput) -> Result<String, AppError> {
        let email = match EmailEntity::new(Some(&input.email)) {
            Ok(email) => email,
            Err(error) => return Err(AppError::new(&error.to_string(), 400)),
        };

        let user = self.users_repository.find_by_email(email).ok_or(AppError::new("User not found", 404))?;

        if user.password != input.password {
            return Err(AppError::new("Invalid password", 401));
        }

        Ok("User logged in".to_string())
    }
}