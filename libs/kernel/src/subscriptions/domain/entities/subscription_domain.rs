/// Value object representing a subscription's domain.
/// 
/// This value object ensures that subscription domains are not empty and provides
/// a type-safe way to handle subscription domain information.

use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct SubscriptionDomain {
    value: String,
}

impl SubscriptionDomain {
    /// Creates a new subscription domain.
    /// 
    /// # Arguments
    /// 
    /// * `val` - The string value of the subscription domain
    /// 
    /// # Returns
    /// 
    /// A `Result` containing either a new `SubscriptionDomain` or a `DomainError`
    /// if the input is empty.
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val.is_empty() {
            return Err(DomainError::ValueObjectError { 
                value: "Subscription domain must not be empty".to_string() 
            });
        }
        Ok(Self { value: val })
    }

    /// Returns the string value of the subscription domain.
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for SubscriptionDomain {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for SubscriptionDomain {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for SubscriptionDomain {}

impl Hash for SubscriptionDomain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
} 