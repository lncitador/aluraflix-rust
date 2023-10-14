use crate::domain::value_objects::unique_id::UniqueEntityID;

pub trait Repository<T> {
    fn find_all(&self) -> Vec<T>;
    fn find_by_id(&self, id: UniqueEntityID) -> Option<T>;
    fn save(&mut self, entity: T);
    fn delete(&self, id: UniqueEntityID) -> bool;
}