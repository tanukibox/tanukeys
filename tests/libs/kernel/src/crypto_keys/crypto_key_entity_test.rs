#[cfg(test)]
pub mod create_simple_crypto_key {
    use crate::crypto_keys::mothers::crypto_key_mother::CryptoKeyMother;


    #[test]
    fn create_simple_key() {
        CryptoKeyMother::random();
    }
}