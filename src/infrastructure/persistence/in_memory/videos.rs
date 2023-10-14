use crate::application::repositories::Repository;
use crate::application::repositories::videos::VideosRepository;
use crate::domain::entities::videos::{Videos};
use crate::domain::value_objects::unique_id::UniqueEntityID;

#[derive(Clone)]
pub struct VideosRepositoryInMemory {
    pub videos: Vec<Videos>,
}

impl VideosRepositoryInMemory {
    pub fn new() -> Self {
        Self { videos: vec![] }
    }

    fn len(&self) -> usize {
        self.videos.len()
    }
}

impl Repository<Videos> for VideosRepositoryInMemory {
    fn find_all(&self) -> Vec<Videos> {
        if self.len() > 10 {
            panic!("Too many videos in memory, please clear the memory before running the tests");
        }

        self.videos.clone()
    }

    fn find_by_id(&self, id: UniqueEntityID) -> Option<Videos> {
        self.videos.iter().find(|v| v.id == id).cloned()
    }

    fn save(&mut self, entity: Videos) {
        self.videos.push(entity);
    }

    fn delete(&mut self, id: UniqueEntityID) -> bool {
        let len = self.len();
        self.videos.retain(|v| v.id != id);
        len != self.len()
    }
}

impl VideosRepository for VideosRepositoryInMemory {
    fn find_by_category_id(&self, category_id: UniqueEntityID) -> Vec<Videos> {
        self.videos.iter().filter(|v| v.category_id == category_id).cloned().collect()
    }
}