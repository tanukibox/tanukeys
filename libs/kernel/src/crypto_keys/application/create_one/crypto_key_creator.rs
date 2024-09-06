use std::sync::Arc;

use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use crate::crypto_keys::domain::events::crypto_key_created_event::CryptoKeyCreatedEvent;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use events::domain::event_bus::EventBus;

pub struct CryptoKeyCreator<R: CryptoKeyRepository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: CryptoKeyRepository, E: EventBus> CryptoKeyCreator<R, E> {
    pub fn new(crypto_key_repository: Arc<R>, event_bus: Arc<E>) -> CryptoKeyCreator<R, E> {
        CryptoKeyCreator { repository: crypto_key_repository, event_bus }
    }

    pub async fn run(&self, id: CryptoKeyId, name: CryptoKeyName, payload: CryptoKeyPayload, user_id: UserId,
                        logged_user: UserId) -> Result<(), DomainError> {
        if user_id != logged_user {
            return Err(DomainError::UserNotAuthorized { user_id: logged_user.value() })
        }
        let key = CryptoKey::new(id.clone(), name.clone(), payload.clone(), user_id.clone());
        let res = self.repository.create_one(&key).await;
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        let created_event = CryptoKeyCreatedEvent::new_shared(id, name, payload, user_id);
        self.event_bus.publish(created_event).await;
        Ok(())
    }
}
