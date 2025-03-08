use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyDescription {
    value: String,
}

impl CryptoKeyDescription {
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val.is_empty() {
            return Err(DomainError::ValueObjectError { 
                value: "Crypto key description must not be empty".to_string() 
            });
        }

        if val.starts_with(' ') {
            return Err(DomainError::ValueObjectError { 
                value: "Crypto key description must not start with a space".to_string() 
            });
        }

        if val.ends_with(' ') {
            return Err(DomainError::ValueObjectError { 
                value: "Crypto key description must not end with a space".to_string() 
            });
        }

        if val.contains("  ") {
            return Err(DomainError::ValueObjectError { 
                value: "Crypto key description must not contain consecutive spaces".to_string() 
            });
        }

        Ok(Self { value: val })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyDescription {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for CryptoKeyDescription {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl Eq for CryptoKeyDescription {}

impl Hash for CryptoKeyDescription {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}