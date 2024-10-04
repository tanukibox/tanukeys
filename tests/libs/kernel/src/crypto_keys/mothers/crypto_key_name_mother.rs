use std::iter;

use kernel::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use rand::Rng;




pub struct CryptoKeyNameMother {}

impl CryptoKeyNameMother {
    pub fn random() -> CryptoKeyName {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        let gen_one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
        let name_size = rand::thread_rng().gen_range(5..15);
        let random_str: String = iter::repeat_with(gen_one_char).take(name_size).collect();
        CryptoKeyName::new(Some(random_str)).unwrap()
    }

    pub fn with_params(value: Option<String>) -> CryptoKeyName {
        match value {
            Some(value) => CryptoKeyName::new(Some(value)).unwrap(),
            None => Self::random(),
        }
    }
}