use std::error::Error;

use async_trait::async_trait;

use super::entities::{user::User, user_id::UserId};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: &UserId) -> Result<User, Box<dyn Error>>;
    async fn create_one(&self, user: &User) -> Result<(), Box<dyn Error>>;
    async fn update_one(&self, user: &User) -> Result<(), Box<dyn Error>>;
    async fn delete_one(&self, id: &UserId) -> Result<(), Box<dyn Error>>;
}
