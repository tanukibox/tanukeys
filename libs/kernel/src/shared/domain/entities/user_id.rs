use std::error::Error;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct UserId {
    value: String,
}

impl UserId {
    pub fn new(id: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self { value: id })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl PartialEq for UserId {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for UserId {}

impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Clone for UserId {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}
