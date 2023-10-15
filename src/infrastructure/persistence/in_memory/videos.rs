use async_trait::async_trait;
use crate::application::repositories::{Repository, RepositoryError};
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

#[async_trait]
impl Repository<Videos> for VideosRepositoryInMemory {
    async fn find_all(&self) -> Vec<Videos> {
        if self.len() > 10 {
            panic!("Too many videos in memory, please clear the memory before running the tests");
        }

        self.videos.clone()
    }

    async fn find_by_id(&self, id: UniqueEntityID) -> Result<Videos, RepositoryError> {
        match self.videos.iter().find(|v| v.id == id) {
            Some(video) => Ok(video.clone()),
            None => Err(RepositoryError::NotFound("Video not found".to_string())),
        }
    }

    async fn save(&mut self, entity: Videos) -> Result<Videos, RepositoryError> {
        match self.videos.iter().find(|v| v.id == entity.id) {
            Some(_) => Err(RepositoryError::AlreadyExists("Video already exists".to_string())),
            None => {
                self.videos.push(entity.clone());
                Ok(entity)
            }
        }
    }

    async fn delete(&mut self, id: UniqueEntityID) -> Result<(), RepositoryError> {
        let len = self.len();
        self.videos.retain(|v| v.id != id);
        if len != self.len() {
            Ok(())
        } else {
            Err(RepositoryError::NotFound("Video not found".to_string()))
        }
    }
}

#[async_trait]
impl VideosRepository for VideosRepositoryInMemory {
    async fn find_by_category_id(&self, category_id: UniqueEntityID) -> Vec<Videos> {
        self.videos.iter().filter(|v| v.category_id == category_id).cloned().collect()
    }
}