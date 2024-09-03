use std::error::Error;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyPayload {
    value: String,
}

impl CryptoKeyPayload {
    pub fn new(id: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self { value: id })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for CryptoKeyPayload {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
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