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

impl Repository<Users> for UsersRepositoryInMemory {
    fn find_all(&self) -> Vec<Users> {
        if self.len() > 10 {
            panic!("Too many users in memory, please clear the memory before running the tests");
        }

        self.users.clone()
    }

    fn find_by_id(&self, id: UniqueEntityID) -> Result<Users, RepositoryError> {
        match self.users.iter().find(|v| v.id == id) {
            Some(user) => Ok(user.clone()),
            None => Err(RepositoryError::NotFound("User not found".to_string())),
        }
    }

    fn save(&mut self, entity: Users) -> Result<Users, RepositoryError> {
        match self.users.iter().find(|v| v.id == entity.id) {
            Some(_) => Err(RepositoryError::AlreadyExists("User already exists".to_string())),
            None => {
                self.users.push(entity.clone());
                Ok(entity)
            }
        }
    }

    fn delete(&mut self, id: UniqueEntityID) -> Result<(), RepositoryError> {
        let len = self.len();
        self.users.retain(|v| v.id != id);
        if len != self.len() {
            Ok(())
        } else {
            Err(RepositoryError::NotFound("User not found".to_string()))
        }
    }
}

impl UsersRepository for UsersRepositoryInMemory {
    fn find_by_email(&self, email: EmailEntity) -> Option<Users> {
        self.users.iter().find(|v| v.email == email).cloned()
    }
}