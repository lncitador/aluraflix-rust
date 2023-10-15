#[cfg(test)]
mod test_auth_use_case {
    use tokio;
    use std::sync::{Arc, Mutex};
    use crate::application::repositories::Repository;
    use crate::application::usecases::authentication::{AuthUseCase, AuthUseCaseError};
    use crate::domain::entities::users::{Users, UsersInput};
    use crate::infrastructure::persistence::in_memory::users::UsersRepositoryInMemory;
    use crate::domain::value_objects::ValueObjectTrait;

    struct Sut {
        users_repository: Arc<Mutex<UsersRepositoryInMemory>>,
        use_case: AuthUseCase,
        initial_user: Users,
    }

    async fn setup_sut() -> Sut {
        let users_repository = Arc::new(Mutex::new(UsersRepositoryInMemory::new()));

        let initial_user = Users::new(&UsersInput {
            name: "John Doe".to_string(),
            email: "doejoe@test.com".to_string(),
            password: "12345678".to_string(),
        }).unwrap();

        users_repository
            .lock()
            .unwrap()
            .save(initial_user.clone())
            .await
            .unwrap();

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

        #[tokio::test]
        async fn it_should_not_sing_in_when_the_user_does_not_exist() {
            let users_repository = Arc::new(Mutex::new(UsersRepositoryInMemory::new()));
            let use_case = AuthUseCase::new(users_repository.clone());

            let input = SignInInput {
                email: "johndoe@test.com".to_string(),
                password: "12345678".to_string(),
            };

            let result = use_case.sign_in(input).await;

            assert!(result.is_err());
        }

        #[tokio::test]
        async fn it_should_not_sing_in_when_the_password_is_invalid() {
            let sut = setup_sut().await;

            let input = SignInInput {
                email: sut.initial_user.email.to_string(),
                password: "123456789".to_string(),
            };

            let result = sut.use_case.sign_in(input).await;

            assert!(result.is_err());
        }

        #[tokio::test]
        async fn it_should_sing_in_when_the_user_exists_and_the_password_is_valid() {
            let sut = setup_sut().await;

            let input = SignInInput {
                email: sut.initial_user.email.to_string(),
                password: sut.initial_user.password.to_string(),
            };

            let result = sut.use_case.sign_in(input).await;

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "User logged in".to_string());
        }

        #[tokio::test]
        async fn it_should_not_sing_in_when_the_email_is_invalid() {
            let sut = setup_sut().await;

            let input = SignInInput {
                email: "johndoe".to_string(),
                password: sut.initial_user.password.to_string(),
            };

            let result = sut.use_case.sign_in(input).await;

            assert!(result.is_err());
        }
    }

    #[cfg(test)]
    mod test_sign_up {
        use super::*;

        #[tokio::test]
        async fn it_should_not_sign_up_when_the_email_is_invalid() {
            let mut sut = setup_sut().await;

            let input = UsersInput {
                name: "John Doe".to_string(),
                email: "johndoe".to_string(),
                password: "12345678".to_string(),
            };

            let result = sut.use_case.sign_up(input).await;

            assert!(result.is_err());
        }

        #[tokio::test]
        async fn it_should_not_sign_up_when_the_user_already_exists() {
            let mut sut = setup_sut().await;

            let input = UsersInput {
                name: "John Doe".to_string(),
                email: sut.initial_user.email.to_string(),
                password: "12345678".to_string(),
            };

            let result = sut.use_case.sign_up(input).await;

            assert!(result.is_err());
            assert!(matches!(result.unwrap_err(), AuthUseCaseError::UserAlreadyExists));
        }

        #[tokio::test]
        async fn it_should_sign_up_when_the_user_does_not_exist() {
            let mut sut = setup_sut().await;

            let input = UsersInput {
                name: "Marie Joe".to_string(),
                email: "joema@test.com".to_string(),
                password: "12345678".to_string(),
            };

            let result = sut.use_case.sign_up(input).await;

            assert!(result.is_ok());
            assert_eq!(result.unwrap().name, "Marie Joe".to_string());
        }
    }
}