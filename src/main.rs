use std::env;
use crate::domain::value_objects::unique_id::UniqueEntityID;
use crate::domain::value_objects::ValueObjectTrait;

mod domain;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let id = UniqueEntityID::new("");

    match id {
        Ok(id) => println!("{}", id.to_string()),
        Err(err) => println!("{}", err.to_string())
    }

    println!("{}", database_url);
}
