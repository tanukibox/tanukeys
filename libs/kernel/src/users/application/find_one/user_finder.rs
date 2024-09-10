use tracing::debug;

use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use crate::users::domain::{
    entities::user::User,
    user_repository::UserRepository,
};
use std::sync::Arc;

pub struct UserFinder<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserFinder<R> {
    pub fn new(user_repository: Arc<R>) -> UserFinder<R> {
        UserFinder { user_repository }
    }

    pub async fn run(&self, user_id: UserId) -> Result<User, DomainError> {
        debug!("Finding user with id: {}", user_id.value());
        let user_res = self.user_repository.find_by_id(&user_id).await;
        if user_res.is_err() {
            debug!("Error finding user with id: {}", user_id.value());
            return Err(user_res.err().unwrap());
        }
        debug!("User with id: {} found", user_id.value());
        user_res
    }
}
