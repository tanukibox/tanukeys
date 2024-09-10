use tracing::debug;

use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use std::sync::Arc;

pub struct CryptoKeyFinder<R: CryptoKeyRepository> {
    repository: Arc<R>,
}

impl<R: CryptoKeyRepository> CryptoKeyFinder<R> {
    pub fn new(crypto_key_repository: Arc<R>) -> CryptoKeyFinder<R> {
        CryptoKeyFinder { repository: crypto_key_repository }
    }

    pub async fn run(&self, key_id: CryptoKeyId, user_id: UserId) -> Result<CryptoKey, DomainError> {
        debug!("Finding crypto key with id: {}", key_id.value());
        let res = self.repository.find_by_id(&user_id, &key_id).await;
        if res.is_err() {
            debug!("Error finding crypto key with id: {}", key_id.value());
            return Err(res.err().unwrap());
        }
        debug!("Crypto key with id: {} found", key_id.value());
        Ok(res.unwrap())
    }
}
