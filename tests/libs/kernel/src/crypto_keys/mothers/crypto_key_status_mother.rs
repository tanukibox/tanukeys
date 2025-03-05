use kernel::crypto_keys::domain::entities::crypto_key_status::CryptoKeyStatus;
use rand::Rng;

pub struct CryptoKeyStatusMother {}

impl CryptoKeyStatusMother {
    pub fn random() -> CryptoKeyStatus {
        let mut rng = rand::thread_rng();
        let status = if rng.gen_bool(0.5) {
            "active"
        } else {
            "revoked"
        };
        CryptoKeyStatus::new(status.to_string()).unwrap()
    }

    pub fn with_params(value: Option<String>) -> CryptoKeyStatus {
        match value {
            Some(value) => CryptoKeyStatus::new(value).unwrap(),
            None => Self::random(),
        }
    }
} 