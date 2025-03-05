use std::iter;
use kernel::crypto_keys::domain::entities::crypto_key_domain::CryptoKeyDomain;
use rand::Rng;

pub struct CryptoKeyDomainMother {}

impl CryptoKeyDomainMother {
    pub fn random() -> CryptoKeyDomain {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        let name_size = rng.gen_range(5..15);
        let random_str: String = iter::repeat_with(|| {
            let mut rng = rand::thread_rng();
            CHARSET[rng.gen_range(0..CHARSET.len())] as char
        }).take(name_size).collect();
        CryptoKeyDomain::new(random_str).unwrap()
    }

    pub fn with_params(value: Option<String>) -> CryptoKeyDomain {
        match value {
            Some(value) => CryptoKeyDomain::new(value).unwrap(),
            None => Self::random(),
        }
    }
} 