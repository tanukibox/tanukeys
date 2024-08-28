use serde::{Deserialize, Serialize};

use crate::users::domain::entities::{user::User, user_id, user_name};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
}

pub fn parse_to_dto(user: &User) -> UserDto {
    UserDto {
        id: user.id.value(),
        name: user.name.value(),
    }
}

pub fn parse_to_domain(dto: &UserDto) -> User {
    let user_id = user_id::UserId::new(dto.id.clone());
    let user_name = user_name::UserName::new(dto.name.clone());
    User::new(user_id, user_name)
}
