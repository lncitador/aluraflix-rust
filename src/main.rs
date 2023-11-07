use std::sync::{Arc, Mutex};
use crate::domain::value_objects::ValueObjectTrait;
use crate::infrastructure::persistence::in_memory::users::UsersRepositoryInMemory;
use crate::application::usecases::authentication::AuthUseCase;
use crate::domain::entities::users::UsersInput;
use crate::infrastructure::persistence::database::users::UsersRepositoryImpl;

mod domain;
mod application;
mod infrastructure;

#[tokio::main]
async fn main() {
    let user_repositories = UsersRepositoryImpl::new().await;

    if user_repositories.is_err() {
        panic!("Error connecting to database");
    }

    let mut use_case = AuthUseCase::new(
        Arc::new(Mutex::new(user_repositories.unwrap()))
    );

    let user_input = UsersInput{
        name: String::from("Teste"),
        email: String::from("johndoe@test.com"),
        password: String::from("12345678")
    };

    let user = use_case.sign_up(user_input).await.unwrap();

    println!("{:?}", user);
}
