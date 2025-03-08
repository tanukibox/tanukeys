use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use crate::shared::domain::entities::user_id::UserId;
use aggregate_root::domain::aggregate_root::AggregateRoot;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::entities::crypto_key_type::CryptoKeyType;
use crate::crypto_keys::domain::entities::crypto_key_domain::CryptoKeyDomain;
use crate::crypto_keys::domain::entities::crypto_key_status::CryptoKeyStatus;
use std::fmt::Debug;

use super::crypto_key_description::CryptoKeyDescription;

#[derive(Debug)]
pub struct CryptoKey {
    pub id: CryptoKeyId,
    pub name: CryptoKeyName,
    pub payload: CryptoKeyPayload,
    pub user_id: UserId,
    pub description: CryptoKeyDescription,
    pub key_type: CryptoKeyType,
    pub domain: CryptoKeyDomain,
    pub status: CryptoKeyStatus,
}

impl CryptoKey {
    pub fn new(
        id: CryptoKeyId,
        name: CryptoKeyName,
        payload: CryptoKeyPayload,
        user_id: UserId,
        description: CryptoKeyDescription,
        key_type: CryptoKeyType,
        domain: CryptoKeyDomain,
        status: CryptoKeyStatus,
    ) -> Self {
        Self { 
            id, 
            name, 
            payload, 
            user_id, 
            description,
            key_type,
            domain,
            status,
        }
    }
}

impl AggregateRoot for CryptoKey {
    fn get_type() -> String {
        "kernel.crypto-key".to_string()
    }
}

impl Clone for CryptoKey {
    fn clone(&self) -> Self {
        Self::new(
            self.id.clone(),
            self.name.clone(),
            self.payload.clone(),
            self.user_id.clone(),
            self.description.clone(),
            self.key_type.clone(),
            self.domain.clone(),
            self.status.clone(),
        )
    }
}

impl PartialEq for CryptoKey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.payload == other.payload &&
        self.user_id == other.user_id &&
        self.description == other.description &&
        self.key_type == other.key_type &&
        self.domain == other.domain &&
        self.status == other.status
    }
}

impl Eq for CryptoKey {}