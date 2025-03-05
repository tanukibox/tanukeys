use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyType {
    value: String,
}

impl CryptoKeyType {
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val == "" {
            return Err(DomainError::ValueObjectError { value: "Crypto key type must not be empty".to_string() })
        }
        Ok(Self { value: val })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyType {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for CryptoKeyType {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for CryptoKeyType {}

impl Hash for CryptoKeyType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
} 