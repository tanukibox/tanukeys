use sqlx::prelude::FromRow;

use crate::users::domain::entities::{user::User, user_id::UserId, user_name::UserName};


#[derive(Debug, FromRow)]
pub struct SqlxUser {   
    pub id: String,
    pub name: String,
}

impl SqlxUser {
    pub fn to_domain(self) -> User {
        User::new(
            UserId::new(self.id).unwrap(),
            UserName::new(self.name).unwrap(),
        )
    }

    pub fn from_domain(user: &User) -> Self {
        Self {
            id: user.id.value(),
            name: user.name.value(),
        }
    }
}