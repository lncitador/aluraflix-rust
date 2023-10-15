#[cfg(test)]
mod test_videos_use_case {
    use std::sync::{Arc, Mutex};
    use crate::application::usecases::videos::VideosUseCase;
    use crate::infrastructure::persistence::in_memory::videos::VideosRepositoryInMemory;

    struct Sut {
        videos_repository: Arc<Mutex<VideosRepositoryInMemory>>,
        use_case: VideosUseCase,
    }

    async fn setup_sut() -> Sut {
        let videos_repository = Arc::new(Mutex::new(VideosRepositoryInMemory::new()));
        let use_case = VideosUseCase::new(videos_repository.clone());

        Sut {
            videos_repository,
            use_case,
        }
    }

    #[cfg(test)]
    mod test_ok {
        #[tokio::test]
        async fn it_should_add_up_to_1_plus_1() {
            assert_eq!(1 + 1, 2);
        }
    }
}