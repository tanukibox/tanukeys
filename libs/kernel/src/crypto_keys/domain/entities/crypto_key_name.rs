use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyName {
    value: String,
}

impl CryptoKeyName {
    pub fn new(val: Option<String>) -> Result<Self, DomainError> {
        if val.is_none() {
            return Err(DomainError::ValueObjectError { value: "Cryptokey name must not be null".to_string() })
        }
        let val = val.unwrap();
        Ok(Self { value: val })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyName {
    fn clone(&self) -> Self {
        Self::new(Some(self.value.clone())).unwrap()
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