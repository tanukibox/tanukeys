use std::{error::Error, sync::Arc};

use events::domain::event_bus::EventBus;
use crate::shared::domain::entities::user_id::UserId;
use crate::users::domain::{
    entities::{user::User, user_name::UserName}, events::user_created_event::UserCreatedEvent, user_repository::UserRepository
};

pub struct UserCreator<R: UserRepository, E: EventBus> {
    user_repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: UserRepository, E: EventBus> UserCreator<R, E> {
    pub fn new(user_repository: Arc<R>, event_bus: Arc<E>) -> UserCreator<R, E> {
        UserCreator { user_repository, event_bus }
    }

    pub async fn run(&self, id: UserId, name: UserName) -> Result<(), Box<dyn Error>> {
        let user = User::new(id, name);
        let res = self.user_repository.create_one(&user).await;
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        let created_event = UserCreatedEvent::new_shared(user.id, user.name);
        self.event_bus.publish(created_event).await;
        Ok(())
    }
}
