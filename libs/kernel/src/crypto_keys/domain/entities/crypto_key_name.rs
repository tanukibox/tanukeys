use std::error::Error;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct CryptoKeyName {
    value: String,
}

impl CryptoKeyName {
    pub fn new(val: String) -> Result<Self, Box<dyn Error>> {
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