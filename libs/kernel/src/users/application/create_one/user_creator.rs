use std::{error::Error, sync::Arc};

use crate::users::domain::{
    entities::{user::User, user_id::UserId, user_name::UserName},
    user_repository::UserRepository,
};

pub struct UserCreator<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserCreator<R> {
    pub fn new(user_repository: Arc<R>) -> UserCreator<R> {
        UserCreator { user_repository }
    }

    pub async fn run(&self, id: UserId, name: UserName) -> Result<(), Box<dyn Error>> {
        let user = User::new(id, name);
        self.user_repository.create_one(&user).await
    }
}
