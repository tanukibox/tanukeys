use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyName {
    value: String,
}

impl CryptoKeyName {
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val == "" {
            return Err(DomainError::ValueObjectError { value: "Crypto key name must not be empty".to_string() })
        }
        Ok(Self { value: val })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyName {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for CryptoKeyName {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl Eq for CryptoKeyName {}

impl Hash for CryptoKeyName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}