use std::iter;
use kernel::crypto_keys::domain::entities::crypto_key_status::CryptoKeyStatus;
use rand::Rng;

pub struct CryptoKeyStatusMother {}

impl CryptoKeyStatusMother {
    pub fn random() -> CryptoKeyStatus {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        let name_size = rng.gen_range(5..15);
        let random_str: String = iter::repeat_with(|| {
            let mut rng = rand::thread_rng();
            CHARSET[rng.gen_range(0..CHARSET.len())] as char
        }).take(name_size).collect();
        CryptoKeyStatus::new(random_str).unwrap()
    }

    pub fn with_params(value: Option<String>) -> CryptoKeyStatus {
        match value {
            Some(value) => CryptoKeyStatus::new(value).unwrap(),
            None => Self::random(),
        }
    }
} 