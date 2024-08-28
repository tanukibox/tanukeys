use std::{error::Error, sync::Arc};

use crate::users::domain::{entities::user_id::UserId, user_repository::UserRepository};

pub struct UserDeleter<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserDeleter<R> {
    pub fn new(user_repository: Arc<R>) -> UserDeleter<R> {
        UserDeleter { user_repository }
    }

    pub async fn run(&self, id: UserId) -> Result<(), Box<dyn Error>> {
        self.user_repository.delete_one(&id).await
    }
}
