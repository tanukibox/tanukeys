use crate::shared::domain::entities::user_id::UserId;
use crate::users::domain::{
    entities::user::User,
    user_repository::UserRepository,
};
use std::{error::Error, sync::Arc};

pub struct UserFinder<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserFinder<R> {
    pub fn new(user_repository: Arc<R>) -> UserFinder<R> {
        UserFinder { user_repository }
    }

    pub async fn run(&self, user_id: UserId) -> Result<User, Box<dyn Error>> {
        self.user_repository.find_by_id(&user_id).await
    }
}
