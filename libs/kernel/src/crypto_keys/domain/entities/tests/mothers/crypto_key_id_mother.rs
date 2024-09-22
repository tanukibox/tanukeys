use uuid::Uuid;

use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;


pub struct CryptoKeyIdMother {}

impl CryptoKeyIdMother {
    pub fn random() -> CryptoKeyId {
        CryptoKeyId::new(Uuid::new_v4().to_string()).unwrap()
    }

    pub fn with_params(value: Option<String>) -> CryptoKeyId {
        match value {
            Some(value) => CryptoKeyId::new(value).unwrap(),
            None => Self::random(),
        }
    }
}