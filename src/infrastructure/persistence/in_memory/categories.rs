use crate::application::repositories::categories::CategoriesRepository;
use crate::application::repositories::Repository;
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

    fn find_by_id(&self, id: UniqueEntityID) -> Option<Categories> {
        self.categories.iter().find(|v| v.id == id).cloned()
    }

    fn save(&mut self, entity: Categories) {
        self.categories.push(entity);
    }

    fn delete(&mut self, id: UniqueEntityID) -> bool {
        let len = self.len();
        self.categories.retain(|v| v.id != id);
        len != self.len()
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