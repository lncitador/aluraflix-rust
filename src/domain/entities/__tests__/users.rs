#[cfg(test)]
mod test_user_entity {
    use crate::domain::entities::users::{Users, UsersInput};

    const NAME: &str = "John Doe";
    const EMAIL: &str = "doejoe@test.com";
    const PASSWORD: &str = "12345678";

    #[test]
    fn it_should_create_a_new_user() {
        let data = UsersInput {
            name: NAME.to_string(),
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };

        let user = Users::new(&data).unwrap();

        assert_eq!(user.name, "John Doe".to_string());
    }

    #[test]
    fn it_should_return_an_error_when_name_is_empty() {
        let data = UsersInput {
            name: "".to_string(),
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };

        let user = Users::new(&data);

        assert!(user.is_err());
    }

    #[test]
    fn it_should_return_an_error_when_name_is_too_short() {
        let data = UsersInput {
            name: "Jo".to_string(),
            email: EMAIL.to_string(),
            password: PASSWORD.to_string(),
        };

        let user = Users::new(&data);

        assert!(user.is_err());
    }

    #[test]
    fn it_should_return_an_error_when_email_is_invalid() {
        let data = UsersInput {
            name: NAME.to_string(),
            email: "doejoe".to_string(),
            password: PASSWORD.to_string(),
        };

        let user = Users::new(&data);

        assert!(user.is_err());
    }

    #[test]
    fn it_should_return_an_error_when_password_is_empty() {
        let data = UsersInput {
            name: NAME.to_string(),
            email: EMAIL.to_string(),
            password: "".to_string(),
        };

        let user = Users::new(&data);

        assert!(user.is_err());
    }
}