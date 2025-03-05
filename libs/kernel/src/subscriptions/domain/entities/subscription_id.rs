use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct SubscriptionId {
    value: String,
}

impl SubscriptionId {
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val.is_empty() {
            return Err(DomainError::ValueObjectError { 
                value: "Subscription ID must not be empty".to_string() 
            });
        }
        Ok(Self { value: val })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for SubscriptionId {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for SubscriptionId {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for SubscriptionId {}

impl Hash for SubscriptionId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
} 