use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyId {
    value: String,
}

impl CryptoKeyId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.contains(" ") {
            return Err(DomainError::ValueObjectError { value: "Cryptokey id must not contain blank spaces".to_string() })
        }
        let contains_upper = id.chars().any(|c| c.is_uppercase());
        if contains_upper {
            return Err(DomainError::ValueObjectError { value: "Cryptokey id must not contain uppercase characters".to_string() })
        }
        if id == "" {
            return Err(DomainError::ValueObjectError { value: "Cryptokey id must not be empty".to_string() })
        }
        Ok(Self { value: id })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyId {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for CryptoKeyId {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for CryptoKeyId {}

impl Hash for CryptoKeyId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
