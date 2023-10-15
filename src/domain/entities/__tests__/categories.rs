#[cfg(test)]
mod test_categories_entity {
    use uuid::Uuid;
    use crate::domain::entities::categories::{Categories, CategoriesInput};

    const NAME: &str = "Category name";
    const COLOR: &str = "#000000";
    const USER_ID: &str = "018b33b3-6d70-7d94-9ecc-0cc5069b30ea";

    #[test]
    fn should_return_error_when_name_is_empty() {
        let input = CategoriesInput {
            name: "".to_string(),
            color: COLOR.to_string(),
            user_id: USER_ID.to_string(),
        };

        let result = Categories::new(&input);

        assert!(result.is_err());
    }

    #[test]
    fn should_return_error_when_name_is_too_short() {
        let input = CategoriesInput {
            name: "a".to_string(),
            color: COLOR.to_string(),
            user_id: USER_ID.to_string(),
        };

        let result = Categories::new(&input);

        assert!(result.is_err());
    }

    #[test]
    fn should_return_error_when_color_is_invalid() {
        let input = CategoriesInput {
            name: NAME.to_string(),
            color: "invalid".to_string(),
            user_id: USER_ID.to_string(),
        };

        let result = Categories::new(&input);

        assert!(result.is_err());
    }

    #[test]
    fn should_return_error_when_user_id_is_invalid() {
        let input = CategoriesInput {
            name: NAME.to_string(),
            color: COLOR.to_string(),
            user_id: "invalid".to_string(),
        };

        let result = Categories::new(&input);

        assert!(result.is_err());
    }

    #[test]
    fn should_return_categories_when_input_is_valid() {
        let input = CategoriesInput {
            name: NAME.to_string(),
            color: COLOR.to_string(),
            user_id: USER_ID.to_string(),
        };

        let result = Categories::new(&input);

        assert!(result.is_ok());
    }
}