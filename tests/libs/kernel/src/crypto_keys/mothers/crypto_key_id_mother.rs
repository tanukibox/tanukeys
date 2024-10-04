use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use uuid::Uuid;



pub struct CryptoKeyIdMother {}

impl CryptoKeyIdMother {
    pub fn random() -> CryptoKeyId {
        CryptoKeyId::new(Some(Uuid::new_v4().to_string())).unwrap()
    }

    pub fn with_params(value: Option<String>) -> CryptoKeyId {
        match value {
            Some(value) => CryptoKeyId::new(Some(value)).unwrap(),
            None => Self::random(),
        }
    }
}