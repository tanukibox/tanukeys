use std::error::Error;

use async_trait::async_trait;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::shared::domain::entities::user_id::UserId;

#[async_trait]
pub trait CryptoKeyRepository: Send + Sync + 'static {
    async fn find_by_id(&self, user_id: &UserId, id: &CryptoKeyId) -> Result<CryptoKey, Box<dyn Error>>;
    async fn create_one(&self, user: &CryptoKey) -> Result<(), Box<dyn Error>>;
    async fn update_one(&self, user: &CryptoKey) -> Result<(), Box<dyn Error>>;
    async fn delete_one(&self, user_id: &UserId, id: &CryptoKeyId) -> Result<(), Box<dyn Error>>;
}
