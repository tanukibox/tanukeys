use crate::shared::domain::entities::user_id::UserId;
use crate::users::domain::entities::user_bio::UserBio;
use crate::users::domain::entities::{user::User, user_name::UserName};
use sqlx::prelude::FromRow;


#[derive(Debug, FromRow)]
pub struct SqlxUser {   
    pub id: String,
    pub name: String,
    pub bio: Option<String>,
}

impl SqlxUser {
    pub fn to_domain(self) -> User {
        User::new(
            UserId::new(Some(self.id)).unwrap(),
            UserName::new(Some(self.name)).unwrap(),
            UserBio::new(self.bio).unwrap(),
        )
    }

    pub fn from_domain(user: &User) -> Self {
        Self {
            id: user.id.value(),
            name: user.name.value(),
            bio: user.bio.value(),
        }
    }
}