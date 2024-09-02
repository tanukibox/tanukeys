use aggregate_root::domain::aggregate_root::AggregateRoot;
use crate::shared::domain::entities::user_id::UserId;
use super::user_name::UserName;

pub struct User {
    pub id: UserId,
    pub name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }
}

impl AggregateRoot for User {
    fn get_type() -> String {
        "kernel.user".to_string()
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        Self::new(self.id.clone(), self.name.clone())
    }
}
