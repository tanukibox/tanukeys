use sqlx::FromRow;
use crate::crypto_keys::domain::entities::{
    crypto_key::CryptoKey,
    crypto_key_description::CryptoKeyDescription,
    crypto_key_id::CryptoKeyId,
    crypto_key_name::CryptoKeyName,
    crypto_key_payload::CryptoKeyPayload,
    crypto_key_type::CryptoKeyType,
    crypto_key_domain::CryptoKeyDomain,
    crypto_key_status::CryptoKeyStatus,
};
use crate::shared::domain::entities::user_id::UserId;

#[derive(Debug, FromRow, Clone)]
pub struct SqlxCryptoKey {
    pub id: String,
    pub name: String,
    pub payload: String,
    pub user_id: String,
    pub description: String,
    pub key_type: String,
    pub domain: String,
    pub status: String,
}

impl SqlxCryptoKey {
    pub fn to_domain(self) -> CryptoKey {
        CryptoKey::new(
            CryptoKeyId::new(self.id).unwrap(),
            CryptoKeyName::new(self.name).unwrap(),
            CryptoKeyPayload::new(self.payload).unwrap(),
            UserId::new(self.user_id).unwrap(),
            CryptoKeyDescription::new(self.description).unwrap(),
            CryptoKeyType::new(self.key_type).unwrap(),
            CryptoKeyDomain::new(self.domain).unwrap(),
            CryptoKeyStatus::new(self.status).unwrap(),
        )
    }

    pub fn from_domain(key: &CryptoKey) -> Self {
        Self {
            id: key.id.value(),
            name: key.name.value(),
            payload: key.payload.value(),
            user_id: key.user_id.value(),
            description: key.description.value(),
            key_type: key.key_type.value(),
            domain: key.domain.value(),
            status: key.status.value(),
        }
    }
}