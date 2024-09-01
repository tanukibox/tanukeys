use std::{error::Error, sync::Arc};

use events::domain::event_bus::EventBus;

use crate::users::domain::{
    entities::{user::User, user_id::UserId, user_name::UserName},
    user_repository::UserRepository,
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
        self.user_repository.create_one(&user).await
    }
}
