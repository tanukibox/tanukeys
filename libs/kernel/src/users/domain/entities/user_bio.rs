use crate::shared::domain::errors::DomainError;

pub struct UserBio {
    value: Option<String>,
}

impl UserBio {
    pub fn new(value: Option<String>) -> Result<UserBio, DomainError> {
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
