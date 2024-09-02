use crate::shared::domain::entities::user_id::UserId;
use events::domain::domain_event::DomainEvent;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;

pub struct CryptoKeyUpdatedEvent {
    pub id: String,
    pub crypto_key_id: CryptoKeyId,
    pub old_crypto_key_id: CryptoKeyId,
    pub crypto_key_name: CryptoKeyName,
    pub old_crypto_key_name: CryptoKeyName,
    pub payload: CryptoKeyPayload,
    pub old_payload: CryptoKeyPayload,
    pub user_id: UserId,
    pub occurred_on: String,
}

impl CryptoKeyUpdatedEvent {
    pub fn new(
        crypto_key_id: CryptoKeyId,
        old_crypto_key_id: CryptoKeyId,
        crypto_key_name: CryptoKeyName,
        old_crypto_key_name: CryptoKeyName,
        payload: CryptoKeyPayload,
        old_payload: CryptoKeyPayload,
        user_id: UserId
    ) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let occurred_on = chrono::Utc::now().to_rfc3339();
        Self { id, crypto_key_id, old_crypto_key_id, crypto_key_name,old_crypto_key_name, payload, old_payload, user_id, occurred_on }
    }

    pub fn new_shared(
        crypto_key_id: CryptoKeyId,
        old_crypto_key_id: CryptoKeyId,
        crypto_key_name: CryptoKeyName,
        old_crypto_key_name: CryptoKeyName,
        payload: CryptoKeyPayload,
        old_payload: CryptoKeyPayload,
        user_id: UserId
    )  -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self::new(crypto_key_id, old_crypto_key_id, crypto_key_name,old_crypto_key_name, payload, old_payload, user_id))
    }
}

impl DomainEvent for CryptoKeyUpdatedEvent {
    fn event_type(&self) -> String {
        "tanukeys.kernel.crypto-keys.updated@1.0.0".to_string()
    }
}