use std::{error::Error, sync::RwLock};

use async_trait::async_trait;

use crate::users::domain::{
    entities::{user::User, user_id::UserId},
    errors::{
        user_already_exists_error::user_already_exists_error,
        user_not_found_error::user_not_found_error,
    },
    user_repository::UserRepository,
};

pub struct MemoryUserRepository {
    users: RwLock<Vec<User>>,
}

impl MemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl UserRepository for MemoryUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<User, Box<dyn Error>> {
        let res = self
            .users
            .read()
            .unwrap()
            .iter()
            .find(|user| user.id.value() == id.value())
            .cloned();
        match res {
            Some(user) => Ok(user.clone()),
            None => Err(Box::new(user_not_found_error(id.clone()))),
        }
    }

    async fn create_one(&self, user: &User) -> Result<(), Box<dyn Error>> {
        let res = self
            .users
            .read()
            .unwrap()
            .iter()
            .find(|stored_user| stored_user.id.value() == user.id.value())
            .cloned();
        match res {
            Some(user) => Err(Box::new(user_already_exists_error(user.id))),
            None => {
                self.users.write().unwrap().push(user.clone());
                Ok(())
            }
        }
    }

    async fn update_one(&self, _user: &User) -> Result<(), Box<dyn Error>> {
        todo!();
    }

    async fn delete_one(&self, id: &UserId) -> Result<(), Box<dyn Error>> {
        let res = self.find_by_id(&id).await;
        match res {
            Ok(_) => {
                self.users
                    .write()
                    .unwrap()
                    .retain(|stored_user| stored_user.id.value() != id.value());
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
