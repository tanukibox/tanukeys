use events::domain::domain_event::DomainEvent;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use crate::shared::domain::entities::user_id::UserId;

pub struct CryptoKeyCreatedEvent {
    pub id: String,
    pub crypto_key_id: CryptoKeyId,
    pub crypto_key_name: CryptoKeyName,
    pub payload: CryptoKeyPayload,
    pub user_id: UserId,
    pub occurred_on: String,
}

impl CryptoKeyCreatedEvent {
    pub fn new(crypto_key_id: CryptoKeyId, crypto_key_name: CryptoKeyName, payload: CryptoKeyPayload, user_id: UserId) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let occurred_on = chrono::Utc::now().to_rfc3339();
        Self { id, crypto_key_id, crypto_key_name, payload, user_id, occurred_on }
    }

    pub fn new_shared(crypto_key_id: CryptoKeyId, crypto_key_name: CryptoKeyName, payload: CryptoKeyPayload, user_id: UserId)  -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self::new(crypto_key_id, crypto_key_name, payload, user_id))
    }
}

impl DomainEvent for CryptoKeyCreatedEvent {
    fn event_type(&self) -> String {
        "tanukeys.kernel.crypto-keys.created@1.0.0".to_string()
    }
}