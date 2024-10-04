use std::iter;

use kernel::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use rand::Rng;




pub struct CryptoKeyPayloadMother {}

impl CryptoKeyPayloadMother {
    pub fn random() -> CryptoKeyPayload {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        let gen_one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
        let name_size = 256;
        let random_str: String = iter::repeat_with(gen_one_char).take(name_size).collect();
        CryptoKeyPayload::new(Some(random_str)).unwrap()
    }

    pub fn with_params(value: Option<String>) -> CryptoKeyPayload {
        match value {
            Some(_) => CryptoKeyPayload::new(value).unwrap(),
            None => Self::random(),
        }
    }
}