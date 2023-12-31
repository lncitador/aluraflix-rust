use async_trait::async_trait;
use crate::application::repositories::{Repository, RepositoryError};
use crate::application::repositories::users::UsersRepository;
use crate::domain::entities::users::Users;
use crate::domain::value_objects::email::EmailEntity;
use crate::domain::value_objects::unique_id::UniqueEntityID;

pub struct UsersRepositoryInMemory {
    pub users: Vec<Users>,
}

impl UsersRepositoryInMemory {
    pub fn new() ->Self {
        Self { users: vec![] }
    }

    fn len(&self) -> usize {
        self.users.len()
    }
}

#[async_trait]
impl Repository<Users> for UsersRepositoryInMemory {
    async fn find_all(&self) -> Vec<Users> {
        if self.len() > 10 {
            panic!("Too many users in memory, please clear the memory before running the tests");
        }

        self.users.clone()
    }

    async fn find_by_id(&self, id: UniqueEntityID) -> Result<Users, RepositoryError> {
        match self.users.iter().find(|v| v.id == id) {
            Some(user) => Ok(user.clone()),
            None => Err(RepositoryError::NotFound("User not found".to_string())),
        }
    }

    async fn save(&mut self, entity: Users) -> Result<Users, RepositoryError> {
        match self.users.iter().find(|v| v.id == entity.id) {
            Some(_) => Err(RepositoryError::AlreadyExists("User already exists".to_string())),
            None => {
                self.users.push(entity.clone());
                Ok(entity)
            }
        }
    }

    async fn delete(&mut self, id: UniqueEntityID) -> Option<RepositoryError> {
        match self.users.iter().position(|v| v.id == id) {
            Some(index) => {
                self.users.remove(index);
                None
            }
            None => Some(RepositoryError::NotFound("User not found".to_string())),
        }
    }
}

#[async_trait]

impl UsersRepository for UsersRepositoryInMemory {
    async fn find_by_email(&self, email: EmailEntity) -> Option<Users> {
        self.users.iter().find(|v| v.email == email).cloned()
    }
}