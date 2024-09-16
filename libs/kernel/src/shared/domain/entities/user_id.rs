use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct UserId {
    value: String,
}

impl UserId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.contains(" ") {
            return Err(DomainError::ValueObjectError { value: "User id must not contain blank spaces".to_string() })
        }
        Ok(Self { value: id })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl PartialEq for UserId {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for UserId {}

impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Clone for UserId {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}
