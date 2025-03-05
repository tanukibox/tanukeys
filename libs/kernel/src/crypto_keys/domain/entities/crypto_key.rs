use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use crate::shared::domain::entities::user_id::UserId;
use aggregate_root::domain::aggregate_root::AggregateRoot;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use std::fmt::Debug;

use super::crypto_key_description::CryptoKeyDescription;

#[derive(Debug)]
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

impl PartialEq for CryptoKey {
    fn eq(&self, other: &Self) -> bool {
        self.id.value() == other.id.value() &&
        self.name.value() == other.name.value() &&
        self.payload.value() == other.payload.value() &&
        self.user_id.value() == other.user_id.value() &&
        self.description.value() == other.description.value()
    }
}

impl Eq for CryptoKey {}