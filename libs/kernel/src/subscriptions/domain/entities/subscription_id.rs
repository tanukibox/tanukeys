/// Value object representing a subscription's unique identifier.
/// 
/// This value object ensures that subscription IDs are not empty and provides
/// a type-safe way to handle subscription identification.

use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct SubscriptionId {
    value: String,
}

impl SubscriptionId {
    /// Creates a new subscription ID.
    /// 
    /// # Arguments
    /// 
    /// * `val` - The string value of the subscription ID
    /// 
    /// # Returns
    /// 
    /// A `Result` containing either a new `SubscriptionId` or a `DomainError`
    /// if the input is empty.
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val.is_empty() {
            return Err(DomainError::ValueObjectError { 
                value: "Subscription ID must not be empty".to_string() 
            });
        }
        Ok(Self { value: val })
    }

    /// Returns the string value of the subscription ID.
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