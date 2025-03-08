#[cfg(test)]
mod crypto_key_id_mother_stress_tests {
    use crate::crypto_keys::mothers::crypto_key_id_mother::CryptoKeyIdMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let id = CryptoKeyIdMother::random();
            assert!(!id.value().is_empty());
        }
    }
} 