use aggregate_root::domain::aggregate_root::AggregateRoot;
use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use crate::shared::domain::entities::user_id::UserId;

pub struct CryptoKey {
    pub payload: CryptoKeyPayload,
    pub user_id: UserId,
}

impl CryptoKey {
    pub fn new(payload: CryptoKeyPayload, user_id: UserId) -> Self {
        Self { payload, user_id }
    }
}

impl AggregateRoot for CryptoKey {
    fn get_type() -> String {
        "kernel.crypto-key".to_string()
    }
}

impl Clone for CryptoKey {
    fn clone(&self) -> Self {
        Self::new(self.payload.clone(), self.user_id.clone())
    }
}