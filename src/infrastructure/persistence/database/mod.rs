pub mod users;

use async_trait::async_trait;

#[async_trait]
pub trait Database {
    async fn connect(url: &str, pool_size: u32) -> Result<Self, sqlx::Error> where Self: Sized;
}