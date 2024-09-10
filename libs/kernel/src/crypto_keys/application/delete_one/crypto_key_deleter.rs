use std::sync::Arc;

use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::events::crypto_key_created_event::CryptoKeyCreatedEvent;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use events::domain::event_bus::EventBus;
use tracing::debug;
use tracing_subscriber::field::debug;

pub struct CryptoKeyDeleter<R: CryptoKeyRepository, E: EventBus> {
    repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: CryptoKeyRepository, E: EventBus> CryptoKeyDeleter<R, E> {
    pub fn new(crypto_key_repository: Arc<R>, event_bus: Arc<E>) -> CryptoKeyDeleter<R, E> {
        CryptoKeyDeleter { repository: crypto_key_repository, event_bus }
    }

    pub async fn run(&self, id: CryptoKeyId, user_id: UserId, logged_user: UserId) -> Result<(), DomainError> {
        debug!("Starting crypto key deletion");
        if user_id != logged_user {
            debug!("User not authorized to delete crypto key with id: {}", id.value());
            return Err(DomainError::UserNotAuthorized { user_id: logged_user.value() })
        }
        let key = self.repository.find_by_id(&user_id, &id).await;
        if key.is_err() {
            debug!("Error finding crypto key with id: {}", id.value());
            return Err(key.err().unwrap());
        }
        let key = key.unwrap();
        let res = self.repository.delete_one(&user_id, &id).await;
        if res.is_err() {
            debug!("Error deleting crypto key with id: {}", id.value());
            return Err(res.err().unwrap());
        }
        let deleted_event = CryptoKeyCreatedEvent::new_shared(id.clone(), key.name, key.payload, user_id);
        self.event_bus.publish(deleted_event).await;
        debug!("Crypto key with id: {} deleted", id.value());
        Ok(())
    }
}
