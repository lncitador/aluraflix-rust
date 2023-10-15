use crate::application::repositories::categories::CategoriesRepository;
use crate::application::repositories::{Repository, RepositoryError};
use crate::domain::entities::categories::Categories;
use crate::domain::value_objects::unique_id::UniqueEntityID;

#[derive(Clone)]
pub struct CategoriesRepositoryInMemory {
    pub categories: Vec<Categories>,
}

impl CategoriesRepositoryInMemory {
    pub fn new() -> Self {
        Self { categories: vec![] }
    }

    fn len(&self) -> usize {
        self.categories.len()
    }
}

impl Repository<Categories> for CategoriesRepositoryInMemory {
    fn find_all(&self) -> Vec<Categories> {
        if self.len() > 10 {
            panic!("Too many categories in memory, please clear the memory before running the tests");
        }

        self.categories.clone()
    }

    fn find_by_id(&self, id: UniqueEntityID) -> Result<Categories, RepositoryError> {
        match self.categories.iter().find(|v| v.id == id) {
            Some(category) => Ok(category.clone()),
            None => Err(RepositoryError::NotFound("Category not found".to_string())),
        }
    }

    fn save(&mut self, entity: Categories) -> Result<Categories, RepositoryError> {
        match self.categories.iter().find(|v| v.id == entity.id) {
            Some(_) => Err(RepositoryError::AlreadyExists("Category already exists".to_string())),
            None => {
                self.categories.push(entity.clone());
                Ok(entity)
            }
        }
    }

    fn delete(&mut self, id: UniqueEntityID) -> Result<(), RepositoryError> {
        let len = self.len();
        self.categories.retain(|v| v.id != id);
        if len != self.len() {
            Ok(())
        } else {
            Err(RepositoryError::NotFound("Category not found".to_string()))
        }
    }
}

impl CategoriesRepository for CategoriesRepositoryInMemory {
    fn find_by_category_id(&self, category_id: UniqueEntityID) -> Vec<Categories> {
        todo!()
    }

    fn find_by_user_id(&self, user_id: UniqueEntityID) -> Vec<Categories> {
        todo!()
    }
}