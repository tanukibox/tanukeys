use std::{error::Error, sync::Arc};
use events::domain::event_bus::EventBus;
use crate::users::domain::{
    entities::{user::User, user_id::UserId, user_name::UserName},
    user_repository::UserRepository,
};
use crate::users::domain::events::user_updated_event::UserUpdatedEvent;

pub struct UserUpdater<R: UserRepository, E: EventBus> {
    user_repository: Arc<R>,
    event_bus: Arc<E>,
}

impl<R: UserRepository, E: EventBus> UserUpdater<R, E> {
    pub fn new(user_repository: Arc<R>, event_bus: Arc<E>) -> UserUpdater<R, E> {
        UserUpdater { user_repository, event_bus }
    }

    pub async fn run(&self, id: UserId, name: UserName) -> Result<(), Box<dyn Error>> {
        let res = self.user_repository.find_by_id(&id).await;
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        let stored_user = res?;

        let user = User::new(id, name);
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
