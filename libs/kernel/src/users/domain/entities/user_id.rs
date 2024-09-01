use std::{hash::{Hash, Hasher}, error::Error};

#[derive(Debug)]
pub struct UserId {
    value: String,
}

impl UserId {
    pub fn new(id: String) -> Result<UserId, Box<dyn Error>> {
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

impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Eq for UserId {}

impl Clone for UserId {
    fn clone(&self) -> Self {
        Self::new(self.value()).unwrap()
    }
}
