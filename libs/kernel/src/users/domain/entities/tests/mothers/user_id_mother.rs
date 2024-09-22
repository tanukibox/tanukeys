use uuid::Uuid;

use crate::shared::domain::entities::user_id::UserId;


pub struct UserIdModer {}

impl UserIdModer {
    pub fn random() -> UserId {
        UserId::new(Uuid::new_v4().to_string()).unwrap()
    }

    pub fn create(value: String) -> UserId {
        UserId::new(value).unwrap()
    }

    pub fn with_params(value: Option<String>) -> UserId {
        match value {
            Some(value) => UserId::new(value).unwrap(),
            None => Self::random(),
        }
    }
}