use tracing::debug;

use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use std::sync::Arc;

pub struct CryptoKeysByUserFinder<R: CryptoKeyRepository> {
    repository: Arc<R>,
}

impl<R: CryptoKeyRepository> CryptoKeysByUserFinder<R> {
    pub fn new(crypto_key_repository: Arc<R>) -> CryptoKeysByUserFinder<R> {
        CryptoKeysByUserFinder { repository: crypto_key_repository }
    }

    pub async fn run(&self, user_id: UserId) -> Result<Vec<CryptoKey>, DomainError> {
        debug!("Finding crypto keys from user with id: {}", user_id.value());
        let res = self.repository.find_many(&user_id).await;
        if res.is_err() {
            debug!("Error finding crypto keys from user with id: {}", user_id.value());
            return Err(res.err().unwrap());
        }
        let res = res.unwrap();
        debug!("{} crypto keys from user with id {} found", res.len(), user_id.value());
        Ok(res)
    }
}
