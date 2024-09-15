use sqlx::FromRow;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::entities::crypto_key_description::CryptoKeyDescription;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use crate::shared::domain::entities::user_id::UserId;

#[derive(Debug, FromRow, Clone)]
pub struct SqlxCryptoKey {
    pub id: String,
    pub name: String,
    pub payload: String,
    pub user_id: String,
    pub description: String,
}

impl SqlxCryptoKey {
    pub fn to_domain(self) -> CryptoKey {
        CryptoKey::new(
            CryptoKeyId::new(self.id).unwrap(),
            CryptoKeyName::new(self.name).unwrap(),
            CryptoKeyPayload::new(self.payload).unwrap(),
            UserId::new(self.user_id).unwrap(),
            CryptoKeyDescription::new(self.description).unwrap(),
        )
    }

    pub fn from_domain(key: &CryptoKey) -> Self {
        Self {
            id: key.id.value(),
            name: key.name.value(),
            payload: key.payload.value(),
            user_id: key.user_id.value(),
            description: key.description.value(),
        }
    }
}