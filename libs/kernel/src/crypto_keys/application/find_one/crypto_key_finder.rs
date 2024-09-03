use std::{error::Error, sync::Arc};
use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::shared::domain::entities::user_id::UserId;

pub struct CryptoKeyFinder<R: CryptoKeyRepository> {
    repository: Arc<R>,
}

impl<R: CryptoKeyRepository> CryptoKeyFinder<R> {
    pub fn new(crypto_key_repository: Arc<R>) -> CryptoKeyFinder<R> {
        CryptoKeyFinder { repository: crypto_key_repository }
    }

    pub async fn run(&self, key_id: CryptoKeyId, user_id: UserId) -> Result<CryptoKey, Box<dyn Error>> {
        self.repository.find_by_id(&user_id, &key_id).await
    }
}
