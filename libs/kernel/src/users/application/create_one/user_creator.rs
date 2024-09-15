use std::sync::Arc;

use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use crate::users::domain::entities::user_bio::UserBio;
use crate::users::domain::{
    entities::{user::User, user_name::UserName}, events::user_created_event::UserCreatedEvent, user_repository::UserRepository
};
use events::domain::event_bus::EventBus;
use tracing::debug;


pub struct UserCreator<R: UserRepository, E: EventBus> {
    user_repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: UserRepository, E: EventBus> UserCreator<R, E> {
    pub fn new(user_repository: Arc<R>, event_bus: Arc<E>) -> UserCreator<R, E> {
        UserCreator { user_repository, event_bus }
    }

    pub async fn run(&self, id: UserId, name: UserName, bio: UserBio, logged_user: UserId) -> Result<(), DomainError> {
        debug!("Starting user creation");
        if id != logged_user {
            debug!("User not authorized to create user with id: {}", id.value());
            return Err(DomainError::UserNotAuthorized { user_id: logged_user.value() })
        }
        let user = User::new(id.clone(), name, bio);
        let res = self.user_repository.create_one(&user).await;
        if res.is_err() {
            debug!("Error creating user with id: {}", id.value());
            return Err(res.err().unwrap());
        }
        let created_event = UserCreatedEvent::new_shared(user.id, user.name);
        self.event_bus.publish(created_event).await;
        debug!("User with id: {} created", id.value());
        Ok(())
    }
}
