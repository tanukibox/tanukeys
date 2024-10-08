use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait CryptoKeyRepository: Send + Sync + 'static {
    async fn find_many(&self, user_id: &UserId) -> Result<Vec<CryptoKey>, DomainError>;
    async fn find_by_id(&self, user_id: &UserId, id: &CryptoKeyId) -> Result<CryptoKey, DomainError>;
    async fn create_one(&self, user: &CryptoKey) -> Result<(), DomainError>;
    async fn update_one(&self, user: &CryptoKey) -> Result<(), DomainError>;
    async fn delete_one(&self, user_id: &UserId, id: &CryptoKeyId) -> Result<(), DomainError>;
}
