use std::iter;

use kernel::crypto_keys::domain::entities::crypto_key_description::CryptoKeyDescription;
use rand::Rng;



pub struct CryptoKeyDescriptionMother {}

impl CryptoKeyDescriptionMother {
    pub fn random() -> CryptoKeyDescription {
        let name_size = rand::thread_rng().gen_range(30..150);

        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz.,()-!@#$%^&*";
        let mut rng = rand::thread_rng();
        let gen_one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;

        let mut random_str: String = iter::repeat_with(gen_one_char)
            .take(name_size)
            .collect::<String>()
            .trim()
            .to_string();

        // Replace double spaces until none remain
        while random_str.contains("  ") {
            random_str = random_str.replace("  ", " ");
        }

        CryptoKeyDescription::new(random_str).unwrap()
    }

    pub fn with_params(value: Option<String>) -> CryptoKeyDescription {
        match value {
            Some(value) => CryptoKeyDescription::new(value).unwrap(),
            None => Self::random(),
        }
    }
}