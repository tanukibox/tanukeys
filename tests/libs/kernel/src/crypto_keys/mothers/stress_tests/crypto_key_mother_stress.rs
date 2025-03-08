#[cfg(test)]
mod crypto_key_mother_stress_tests {
    use crate::crypto_keys::mothers::crypto_key_mother::CryptoKeyMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let key = CryptoKeyMother::random();
            assert!(!key.id.value().is_empty());
            assert!(!key.name.value().is_empty());
            assert!(!key.payload.value().is_empty());
            assert!(!key.description.value().is_empty());
            assert!(!key.key_type.value().is_empty());
            assert!(!key.domain.value().is_empty());
            assert!(!key.status.value().is_empty());
        }
    }
} 