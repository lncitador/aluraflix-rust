use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::application::repositories::Repository;
use crate::domain::entities::users::Users;
use crate::domain::value_objects::email::EmailEntity;

#[async_trait]
pub trait UsersRepository: Repository<Users> {
    async fn find_by_email(&self, email: EmailEntity) -> Option<Users>;
}

pub type UsersRepositoryContract = Arc<Mutex<dyn UsersRepository>>;