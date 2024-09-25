use crate::shared::domain::errors::DomainError;

pub struct UserBio {
    value: Option<String>,
}

impl UserBio {
    pub fn new(value: Option<String>) -> Result<UserBio, DomainError> {
        if let Some(value) = value.clone() {
            if value.len() > 600 {
                return Err(DomainError::ValueObjectError { value: "User bio cannot be longer than 600 characters".to_string() });
            }
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }
}

impl Clone for UserBio {
    fn clone(&self) -> Self {
        let res = Self::new(self.value());
        res.unwrap()
    }
}
