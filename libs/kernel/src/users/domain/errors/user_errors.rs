#[derive(Debug)]
pub enum UserErrorTypes {
    UserAlreadyExists,
    UserNotFound,
}

impl UserErrorTypes {
    pub fn to_string(&self) -> String {
        match self {
            UserErrorTypes::UserAlreadyExists => "UserAlreadyExists".to_string(),
            UserErrorTypes::UserNotFound => "UserNotFound".to_string(),
        }
    }
}
