use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use crate::shared::domain::entities::user_id::UserId;
use aggregate_root::domain::aggregate_root::AggregateRoot;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;

use super::crypto_key_description::CryptoKeyDescription;

pub struct CryptoKey {
    pub id: CryptoKeyId,
    pub name: CryptoKeyName,
    pub payload: CryptoKeyPayload,
    pub user_id: UserId,
    pub description: CryptoKeyDescription,
}

impl CryptoKey {
    pub fn new(id: CryptoKeyId,name: CryptoKeyName, payload: CryptoKeyPayload, user_id: UserId, description: CryptoKeyDescription) -> Self {
        Self { id, name, payload, user_id, description }
    }
}

impl AggregateRoot for CryptoKey {
    fn get_type() -> String {
        "kernel.crypto-key".to_string()
    }
}

impl Clone for CryptoKey {
    fn clone(&self) -> Self {
        Self::new(self.id.clone(), self.name.clone(), self.payload.clone(), self.user_id.clone(), self.description.clone())
    }
}