use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyDomain {
    value: String,
}

impl CryptoKeyDomain {
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val.is_empty() {
            return Err(DomainError::ValueObjectError { 
                value: "Crypto key domain must not be empty".to_string() 
            });
        }

        if !val.chars().all(|c| c.is_alphanumeric() || c == '.') {
            return Err(DomainError::ValueObjectError { 
                value: "Crypto key domain must only contain alphanumeric characters and dots".to_string() 
            });
        }

        Ok(Self { value: val })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyDomain {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for CryptoKeyDomain {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for CryptoKeyDomain {}

impl Hash for CryptoKeyDomain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
} 