pub mod mothers;

#[cfg(test)]
mod create_simple_crypto_key {

    use super::mothers::crypto_key_mother::CryptoKeyMother;

    #[test]
    fn create_simple_key() {
        CryptoKeyMother::random();
    }
}