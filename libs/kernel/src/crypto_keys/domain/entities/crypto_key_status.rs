use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyStatus {
    value: String,
}

impl CryptoKeyStatus {
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val == "" {
            return Err(DomainError::ValueObjectError { value: "Crypto key status must not be empty".to_string() })
        }
        Ok(Self { value: val })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyStatus {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for CryptoKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for CryptoKeyStatus {}

impl Hash for CryptoKeyStatus {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
} 