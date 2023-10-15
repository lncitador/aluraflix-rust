use async_trait::async_trait;
use crate::application::repositories::Repository;
use crate::domain::entities::categories::Categories;
use crate::domain::value_objects::unique_id::UniqueEntityID;

#[async_trait]
pub trait CategoriesRepository: Repository<Categories> {
    async fn find_by_category_id(&self, category_id: UniqueEntityID) -> Vec<Categories>;
    async fn find_by_user_id(&self, user_id: UniqueEntityID) -> Vec<Categories>;
}