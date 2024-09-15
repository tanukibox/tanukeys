use super::{user_bio::UserBio, user_name::UserName};
use crate::shared::domain::entities::user_id::UserId;
use aggregate_root::domain::aggregate_root::AggregateRoot;

pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub bio: UserBio,
}

impl User {
    pub fn new(id: UserId, name: UserName, bio: UserBio) -> Self {
        Self { id, name, bio }
    }
}

impl AggregateRoot for User {
    fn get_type() -> String {
        "kernel.user".to_string()
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self::new(self.id.clone(), self.name.clone(), self.bio.clone())
    }
}
