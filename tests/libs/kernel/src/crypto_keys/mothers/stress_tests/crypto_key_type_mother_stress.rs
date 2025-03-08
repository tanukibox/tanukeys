#[cfg(test)]
mod crypto_key_type_mother_stress_tests {
    use crate::crypto_keys::mothers::crypto_key_type_mother::CryptoKeyTypeMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let key_type = CryptoKeyTypeMother::random();
            assert!(!key_type.value().is_empty());
        }
    }
} 