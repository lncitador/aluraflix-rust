use std::env;
use crate::domain::entities::categories::{Categories, CategoriesInput};
use crate::domain::entities::videos::{Videos, VideosInput};
use crate::domain::value_objects::unique_id::UniqueEntityID;
use crate::domain::value_objects::ValueObjectTrait;

mod domain;

fn main() {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://postgres:postgres@db:6011/aluraflix-dev"));

    let id = UniqueEntityID::new(Some("018b2444-d876-751b-b1f0-838ecea39578"));

    match id {
        Ok(id) => println!("{}", id.to_string()),
        Err(err) => println!("{}", err.to_string())
    }

    println!("{}", database_url);

    let category_input = &CategoriesInput{
        name: String::from("Teste"),
        color: String::from("rgb(255, 255, 255)"),
        user_id: String::from("018b2444-d876-751b-b1f0-838ecea39578")
    };

    let category = match Categories::new(category_input) {
        Ok(category) => {
            println!("{:#?}", category);

            category
        },
        Err(err) => {
            println!("{}", err.to_string());

            return;
        }
    };

    let input = &VideosInput{
        title: String::from("Teste"),
        description: String::from("Teste"),
        url: String::from("https://www.youtube.com"),
        user_id: String::from("018b2444-d876-751b-b1f0-838ecea39578"),
        category_id: category.id.to_string()
    };

    let video = Videos::new( input);

    match video {
        Ok(video) => println!("{:#?}", video),
        Err(err) => println!("{}", err.to_string())
    }
}
