use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyPayload {
    value: String,
}

impl CryptoKeyPayload {
    pub fn new(value: Option<String>) -> Result<Self, DomainError> {
        if value.is_none() {
            return Err(DomainError::ValueObjectError { value: "Cryptokey payload must not be null".to_string() })
        }
        let value = value.unwrap();
        Ok(Self { value })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyPayload {
    fn clone(&self) -> Self {
        Self::new(Some(self.value.clone())).unwrap()
    }
}

impl PartialEq for CryptoKeyPayload {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl Eq for CryptoKeyPayload {}

impl Hash for CryptoKeyPayload {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}