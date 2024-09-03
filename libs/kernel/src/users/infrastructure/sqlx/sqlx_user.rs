use crate::shared::domain::entities::user_id::UserId;
use crate::users::domain::entities::{user::User, user_name::UserName};
use sqlx::prelude::FromRow;


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