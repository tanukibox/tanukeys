use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use crate::users::domain::events::user_deleted_event::UserDeletedEvent;
use crate::users::domain::user_repository::UserRepository;
use events::domain::event_bus::EventBus;
use tracing::debug;
use std::sync::Arc;

pub struct UserDeleter<R: UserRepository, E: EventBus> {
    user_repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: UserRepository, E: EventBus> UserDeleter<R, E> {
    pub fn new(user_repository: Arc<R>, event_bus: Arc<E>) -> UserDeleter<R, E> {
        UserDeleter { user_repository, event_bus }
    }

    pub async fn run(&self, id: UserId, logged_user: UserId) -> Result<(), DomainError> {
        debug!("Starting user deletion");
        if id != logged_user {
            debug!("User not authorized to delete user with id: {}", id.value());
            return Err(DomainError::UserNotAuthorized { user_id: logged_user.value() })
        }
        let res = self.user_repository.find_by_id(&id).await;
        if res.is_err() {
            debug!("Error finding user with id: {}", id.value());
            return Err(res.err().unwrap());
        }
        let stored_user = res?;
        let res = self.user_repository.delete_one(&id).await;
        if res.is_err() {
            debug!("Error deleting user with id: {}", id.value());
            return Err(res.err().unwrap())
        }
        let deleted_event = UserDeletedEvent::new_shared(id.clone(), stored_user.name);
        self.event_bus.publish(deleted_event).await;
        debug!("User with id: {} deleted", id.value());
        Ok(())
    }
}
