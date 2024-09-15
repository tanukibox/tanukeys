use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use crate::users::domain::entities::user_bio::UserBio;
use crate::users::domain::events::user_updated_event::UserUpdatedEvent;
use crate::users::domain::{
    entities::{user::User, user_name::UserName},
    user_repository::UserRepository,
};
use events::domain::event_bus::EventBus;
use tracing::debug;
use std::sync::Arc;

pub struct UserUpdater<R: UserRepository, E: EventBus> {
    user_repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: UserRepository, E: EventBus> UserUpdater<R, E> {
    pub fn new(user_repository: Arc<R>, event_bus: Arc<E>) -> UserUpdater<R, E> {
        UserUpdater { user_repository, event_bus }
    }

    pub async fn run(&self, id: UserId, name: UserName, bio: UserBio, logged_user: UserId) -> Result<(), DomainError> {
        debug!("Starting user update");
        if id != logged_user {
            return Err(DomainError::UserNotAuthorized { user_id: logged_user.value() })
        }
        let res = self.user_repository.find_by_id(&id).await;
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        let stored_user = res?;

        let user = User::new(id, name, bio);
        let res = self.user_repository.update_one(&user).await;
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        let updated_event = UserUpdatedEvent::new_shared(user.id, user.name,
            stored_user.name);
        self.event_bus.publish(updated_event).await;
        Ok(())
    }
}
