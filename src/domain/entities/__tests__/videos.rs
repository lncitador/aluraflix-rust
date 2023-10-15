#[cfg(test)]
mod test_videos_entity {
    use crate::domain::entities::videos::{Videos, VideosInput};

    const TITLE: &str = "Title";
    const DESCRIPTION: &str = "Description";
    const URL: &str = "https://www.youtube.com/watch?v=6n3pFFPSlW4";
    const CATEGORY_ID: &str = "018b33b7-5b9a-72a7-942f-8c46275aeacd";
    const USER_ID: &str = "018b33b7-c8dd-76a2-98b5-d621862882a8";

    #[test]
    fn should_create_a_new_video() {
        let video_input = VideosInput {
            title: TITLE.to_string(),
            description: DESCRIPTION.to_string(),
            url: URL.to_string(),
            category_id: CATEGORY_ID.to_string(),
            user_id: USER_ID.to_string(),
        };

        let video = Videos::new(&video_input);

        assert!(video.is_ok())
    }

    #[test]
    fn should_return_an_error_when_title_is_empty() {
        let video_input = VideosInput {
            title: "".to_string(),
            description: DESCRIPTION.to_string(),
            url: URL.to_string(),
            category_id: CATEGORY_ID.to_string(),
            user_id: USER_ID.to_string(),
        };

        let video = Videos::new(&video_input);

        assert!(video.is_err());
    }

    #[test]
    fn should_return_an_error_when_title_is_too_short() {
        let video_input = VideosInput {
            title: "a".to_string(),
            description: DESCRIPTION.to_string(),
            url: URL.to_string(),
            category_id: CATEGORY_ID.to_string(),
            user_id: USER_ID.to_string(),
        };

        let video = Videos::new(&video_input);

        assert!(video.is_err());
    }

    #[test]
    fn should_return_an_error_when_description_is_empty() {
        let video_input = VideosInput {
            title: TITLE.to_string(),
            description: "".to_string(),
            url: URL.to_string(),
            category_id: CATEGORY_ID.to_string(),
            user_id: USER_ID.to_string(),
        };

        let video = Videos::new(&video_input);

        assert!(video.is_err());
    }

    #[test]
    fn should_return_an_error_when_description_is_too_short() {
        let video_input = VideosInput {
            title: TITLE.to_string(),
            description: "a".to_string(),
            url: URL.to_string(),
            category_id: CATEGORY_ID.to_string(),
            user_id: USER_ID.to_string(),
        };

        let video = Videos::new(&video_input);

        assert!(video.is_err());
    }

    #[test]
    fn should_return_an_error_when_url_is_invalid() {
        let video_input = VideosInput {
            title: TITLE.to_string(),
            description: DESCRIPTION.to_string(),
            url: "invalid_url".to_string(),
            category_id: CATEGORY_ID.to_string(),
            user_id: USER_ID.to_string(),
        };

        let video = Videos::new(&video_input);

        assert!(video.is_err());
    }

    #[test]
    fn should_return_an_error_when_category_id_is_invalid() {
        let video_input = VideosInput {
            title: TITLE.to_string(),
            description: DESCRIPTION.to_string(),
            url: URL.to_string(),
            category_id: "invalid_category_id".to_string(),
            user_id: USER_ID.to_string(),
        };

        let video = Videos::new(&video_input);

        assert!(video.is_err());
    }

    #[test]
    fn should_return_an_error_when_user_id_is_invalid() {
        let video_input = VideosInput {
            title: TITLE.to_string(),
            description: DESCRIPTION.to_string(),
            url: URL.to_string(),
            category_id: CATEGORY_ID.to_string(),
            user_id: "invalid_user_id".to_string(),
        };

        let video = Videos::new(&video_input);

        assert!(video.is_err());
    }
}