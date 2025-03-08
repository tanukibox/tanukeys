use crate::shared::domain::entities::user_id::UserId;
use crate::crypto_keys::domain::entities::{
    crypto_key_domain::CryptoKeyDomain,
    crypto_key_id::CryptoKeyId,
};
use crate::subscriptions::domain::entities::external_domain::ExternalDomain;
use aggregate_root::domain::aggregate_root::AggregateRoot;

#[derive(Debug)]
pub struct Subscription {
    pub user_id: UserId,
    pub key_domain: CryptoKeyDomain,
    pub external_domain: ExternalDomain,
    pub crypto_key_id: CryptoKeyId,
}

impl Subscription {
    pub fn new(
        user_id: UserId,
        key_domain: CryptoKeyDomain,
        external_domain: ExternalDomain,
        crypto_key_id: CryptoKeyId,
    ) -> Self {
        Self {
            user_id,
            key_domain,
            external_domain,
            crypto_key_id,
        }
    }
}

impl AggregateRoot for Subscription {
    fn get_type() -> String {
        "kernel.subscription".to_string()
    }
}

impl Clone for Subscription {
    fn clone(&self) -> Self {
        Self::new(
            self.user_id.clone(),
            self.key_domain.clone(),
            self.external_domain.clone(),
            self.crypto_key_id.clone(),
        )
    }
}

impl PartialEq for Subscription {
    fn eq(&self, other: &Self) -> bool {
        self.user_id.value() == other.user_id.value() &&
        self.key_domain.value() == other.key_domain.value() &&
        self.external_domain.value() == other.external_domain.value() &&
        self.crypto_key_id.value() == other.crypto_key_id.value()
    }
}

impl Eq for Subscription {} 