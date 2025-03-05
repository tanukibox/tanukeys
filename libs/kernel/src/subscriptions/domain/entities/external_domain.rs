/// Value object representing a subscription's external domain.
/// 
/// This value object ensures that external domains are not empty and provides
/// a type-safe way to handle external domain information.

use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct ExternalDomain {
    value: String,
}

impl ExternalDomain {
    /// Creates a new external domain.
    /// 
    /// # Arguments
    /// 
    /// * `val` - The string value of the external domain
    /// 
    /// # Returns
    /// 
    /// A `Result` containing either a new `ExternalDomain` or a `DomainError`
    /// if the input is empty.
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val.is_empty() {
            return Err(DomainError::ValueObjectError { 
                value: "External domain must not be empty".to_string() 
            });
        }
        Ok(Self { value: val })
    }

    /// Returns the string value of the external domain.
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for ExternalDomain {
    fn clone(&self) -> Self {
        Self::new(self.value.clone()).unwrap()
    }
}

impl PartialEq for ExternalDomain {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for ExternalDomain {}

impl Hash for ExternalDomain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
} 