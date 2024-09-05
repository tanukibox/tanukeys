use super::entities::user::User;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::types::DynError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: &UserId) -> Result<User, DynError>;
    async fn create_one(&self, user: &User) -> Result<(), DynError>;
    async fn update_one(&self, user: &User) -> Result<(), DynError>;
    async fn delete_one(&self, id: &UserId) -> Result<(), DynError>;
}
