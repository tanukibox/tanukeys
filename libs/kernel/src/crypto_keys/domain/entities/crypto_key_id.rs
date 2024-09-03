use std::error::Error;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyId {
    value: String,
}

impl CryptoKeyId {
    pub fn new(id: String) -> Result<Self, Box<dyn Error>> {
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
