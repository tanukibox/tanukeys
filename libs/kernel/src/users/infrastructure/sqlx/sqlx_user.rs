use sqlx::prelude::FromRow;

use crate::users::domain::entities::{user::User, user_id::UserId, user_name::UserName};


#[derive(Debug, FromRow)]
pub struct SqlxUser {   
    id: String,
    name: String,
}

impl SqlxUser {
    pub fn to_domain(self) -> User {
        User::new(
            UserId::new(self.id),
            UserName::new(self.name),
        )
    }
}