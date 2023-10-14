#[cfg(test)]
mod test_auth_use_case {
    use std::rc::Rc;
    use crate::application::repositories::Repository;
    use crate::application::usecases::authentication::AuthUseCase;
    use crate::domain::entities::users::{Users, UsersInput};
    use crate::infrastructure::persistence::in_memory::users::UsersRepositoryInMemory;
    use crate::domain::value_objects::ValueObjectTrait;

    struct Sut {
        users_repository: Rc<UsersRepositoryInMemory>,
        use_case: AuthUseCase,
        initial_user: Users,
    }

    fn setup_sut() -> Sut {
        let mut users_repository = Rc::new(UsersRepositoryInMemory::new());

        let initial_user = Users::new(&UsersInput {
            name: "John Doe".to_string(),
            email: "doejoe@test.com".to_string(),
            password: "12345678".to_string(),
        }).unwrap();

        Rc::get_mut(&mut users_repository).unwrap().save(initial_user.clone());

        let use_case = AuthUseCase::new(users_repository.clone());

        Sut {
            users_repository,
            use_case,
            initial_user,
        }
    }

    #[cfg(test)]
    mod test_sign_in {
        use crate::application::usecases::authentication::SignInInput;
        use super::*;
        #[test]
        fn it_should_not_sing_in_when_the_user_does_not_exist() {
            let users_repository = Rc::new(UsersRepositoryInMemory::new());
            let use_case = AuthUseCase::new(users_repository);

            let input = SignInInput {
                email: "johndoe@test.com".to_string(),
                password: "12345678".to_string(),
            };

            let result = use_case.sign_in(input);

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), 404);
        }

        #[test]
        fn it_should_not_sing_in_when_the_password_is_invalid() {
            let sut = setup_sut();

            let input = SignInInput {
                email: sut.initial_user.email.to_string(),
                password: "123456789".to_string(),
            };

            let result = sut.use_case.sign_in(input);

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), 401);
        }

        #[test]
        fn it_should_sing_in_when_the_user_exists_and_the_password_is_valid() {
            let sut = setup_sut();

            let input = SignInInput {
                email: sut.initial_user.email.to_string(),
                password: sut.initial_user.password.to_string(),
            };

            let result = sut.use_case.sign_in(input);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "User logged in".to_string());
        }

        #[test]
        fn it_should_not_sing_in_when_the_email_is_invalid() {
            let sut = setup_sut();

            let input = SignInInput {
                email: "johndoe".to_string(),
                password: sut.initial_user.password.to_string(),
            };

            let result = sut.use_case.sign_in(input);

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().code(), 400);
        }
    }
}