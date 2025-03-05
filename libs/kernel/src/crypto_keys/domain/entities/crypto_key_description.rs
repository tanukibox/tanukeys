use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyDescription {
    value: String,
}

impl CryptoKeyDescription {
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id == "" {
            return Err(DomainError::ValueObjectError { value: "Crypto key description must not be empty".to_string() })
        }
        Ok(Self { value: id })
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