use crate::shared::domain::errors::DomainError;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CryptoKeyStatusValue {
    Active,
    Revoked,
}

impl CryptoKeyStatusValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            CryptoKeyStatusValue::Active => "active",
            CryptoKeyStatusValue::Revoked => "revoked",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "active" => Ok(CryptoKeyStatusValue::Active),
            "revoked" => Ok(CryptoKeyStatusValue::Revoked),
            _ => Err(DomainError::ValueObjectError { 
                value: format!("Invalid crypto key status: {}. Valid values are: active, revoked", s) 
            }),
        }
    }
}

#[derive(Debug)]
pub struct CryptoKeyStatus {
    value: CryptoKeyStatusValue,
}

impl CryptoKeyStatus {
    pub fn new(val: String) -> Result<Self, DomainError> {
        if val.is_empty() {
            return Err(DomainError::ValueObjectError { 
                value: "Crypto key status must not be empty".to_string() 
            });
        }
        Ok(Self { 
            value: CryptoKeyStatusValue::from_str(&val)? 
        })
    }

    pub fn value(&self) -> String {
        self.value.as_str().to_string()
    }

    pub fn is_active(&self) -> bool {
        matches!(self.value, CryptoKeyStatusValue::Active)
    }

    pub fn is_revoked(&self) -> bool {
        matches!(self.value, CryptoKeyStatusValue::Revoked)
    }
}

impl Clone for CryptoKeyStatus {
    fn clone(&self) -> Self {
        Self { 
            value: self.value.clone() 
        }
    }
}

impl PartialEq for CryptoKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for CryptoKeyStatus {}

impl Hash for CryptoKeyStatus {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
} 