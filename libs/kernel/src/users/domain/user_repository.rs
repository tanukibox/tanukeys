use super::entities::user::User;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: &UserId) -> Result<User, DomainError>;
    async fn create_one(&self, user: &User) -> Result<(), DomainError>;
    async fn update_one(&self, user: &User) -> Result<(), DomainError>;
    async fn delete_one(&self, id: &UserId) -> Result<(), DomainError>;
}
