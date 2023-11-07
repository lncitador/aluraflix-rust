use std::env;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use time::Date;
use uuid::Uuid;
use crate::application::repositories::{Repository, RepositoryError};
use crate::application::repositories::users::UsersRepository;
use crate::domain::entities::users::Users;
use crate::domain::value_objects::email::EmailEntity;
use crate::domain::value_objects::unique_id::UniqueEntityID;
use crate::infrastructure::persistence::database::Database;
use crate::domain::value_objects::ValueObjectTrait;

pub struct UsersRepositoryImpl {
    pub pool: PgPool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UsersModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Date,
    pub updated_at: Date,
}

#[async_trait]
impl Database for UsersRepositoryImpl {
    async fn connect(url: &str, pool_size: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?;

        Ok(Self { pool })
    }
}

impl UsersRepositoryImpl {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL").unwrap_or("postgres://docker:docker@localhost:5432/postgres?schema=public&sslmode=disable".to_string());
        let pool_size = env::var("POOL_SIZE").unwrap_or("10".to_string()).parse::<u32>().unwrap();

        Self::connect(&database_url, pool_size).await
    }
}

#[async_trait]
impl Repository<Users> for UsersRepositoryImpl {
    async fn find_all(&self) -> Vec<Users> {
        todo!()
    }

    async fn find_by_id(&self, id: UniqueEntityID) -> Result<Users, RepositoryError> {
        todo!()
    }

    async fn save(&mut self, entity: Users) -> Result<Users, RepositoryError> {
        let model = sqlx::query_as::<_, UsersModel>(
            r#"
            INSERT INTO users (id, name, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
            .bind(entity.id.to_string())
            .bind(entity.name)
            .bind(entity.email.to_string())
            .bind(entity.password)
            .bind(entity.created_at)
            .bind(entity.updated_at)
            .fetch_one(&self.pool)
            .await;

        match model {
            Ok(data) => Ok(Users::from(data)),
            Err(err) => Some(RepositoryError::Domain(err.into())),
        }
    }

    async fn delete(&mut self, id: UniqueEntityID) -> Option<RepositoryError> {

    }
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn find_by_email(&self, email: EmailEntity) -> Option<Users> {
        let model = sqlx::query_as::<_, UsersModel>(
            r#"
            SELECT id, name, email, password, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
            .bind(email.to_string())
            .fetch_optional(&self.pool)
            .await;

        match model {
            Ok(model) => {
                match model {
                    Some(model) => Some(Users::from(model)),
                    None => None,
                }
            }
            Err(_) => None,
        }
    }
}