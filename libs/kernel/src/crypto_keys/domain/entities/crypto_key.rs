use aggregate_root::domain::aggregate_root::AggregateRoot;
use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;

pub struct CryptoKey {
    pub payload: CryptoKeyPayload,
}

impl CryptoKey {
    pub fn new(payload: CryptoKeyPayload) -> Self {
        Self { payload }
    }
}

impl AggregateRoot for CryptoKey {
    fn get_type() -> String {
        "kernel.crypto-key".to_string()
    }
}

impl Clone for CryptoKey {
    fn clone(&self) -> Self {
        Self::new(self.payload.clone())
    }
}