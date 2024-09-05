use crate::shared::domain::entities::user_id;
use crate::shared::domain::types::DynError;
use crate::users::domain::entities::{user::User, user_name};
use serde::{Deserialize, Serialize};

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

pub fn parse_to_domain(dto: &UserDto) -> Result<User, DynError> {
    let user_id = user_id::UserId::new(dto.id.clone());
    if user_id.is_err() {
        return Err(user_id.err().unwrap());
    }
    let user_id = user_id.unwrap();
    let user_name = user_name::UserName::new(dto.name.clone());
    if user_name.is_err() {
        return Err(user_name.err().unwrap());
    }
    let user_name = user_name.unwrap();
    Ok(User::new(user_id, user_name))
}
