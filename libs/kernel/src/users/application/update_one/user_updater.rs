use std::{error::Error, sync::Arc};

use crate::users::domain::{
    entities::{user::User, user_id::UserId, user_name::UserName},
    user_repository::UserRepository,
};

pub struct UserUpdater<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserUpdater<R> {
    pub fn new(user_repository: Arc<R>) -> UserUpdater<R> {
        UserUpdater { user_repository }
    }

    pub async fn run(&self, id: UserId, name: UserName) -> Result<(), Box<dyn Error>> {
        let user = User::new(id, name);
        self.user_repository.update_one(&user).await
    }
}
