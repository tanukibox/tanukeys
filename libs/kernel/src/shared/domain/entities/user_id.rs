use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct UserId {
    value: String,
}

impl UserId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id == "" {
            return Err(DomainError::ValueObjectError { value: "User id must not be empty".to_string() })
        }
        if id.contains(" ") {
            return Err(DomainError::ValueObjectError { value: "User id must not contain blank spaces".to_string() })
        }
        let contains_upper = id.chars().any(|c| c.is_uppercase());
        if contains_upper {
            return Err(DomainError::ValueObjectError { value: "User id must not contain uppercase characters".to_string() })
        }
        const ALLOWED_CHARS: &str = "abcdefghijklmnopqrstuvwxyz0123456789_.-";
        let contains_not_alloweed_char = id.chars().any(|c| !ALLOWED_CHARS.contains(c));
        if contains_not_alloweed_char {
            return Err(DomainError::ValueObjectError { value: "User id must contain only alphanumeric characters or <-._>.".to_string() })
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
