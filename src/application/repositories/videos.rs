use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::application::repositories::Repository;
use crate::domain::entities::videos::Videos;
use crate::domain::value_objects::unique_id::UniqueEntityID;

#[async_trait]
pub trait VideosRepository: Repository<Videos> {
   async fn find_by_category_id(&self, category_id: UniqueEntityID) -> Vec<Videos>;
}

pub type VideosRepositoryContract =  Arc<Mutex<dyn VideosRepository>>;