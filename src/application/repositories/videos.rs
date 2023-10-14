use crate::application::repositories::Repository;
use crate::domain::entities::videos::Videos;
use crate::domain::value_objects::unique_id::UniqueEntityID;

pub trait VideosRepository: Repository<Videos> {
    fn find_by_category_id(&self, category_id: UniqueEntityID) -> Vec<Videos>;
}
