#[cfg(test)]
mod crypto_key_name_mother_stress_tests {
    use crate::crypto_keys::mothers::crypto_key_name_mother::CryptoKeyNameMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let name = CryptoKeyNameMother::random();
            assert!(!name.value().is_empty());
        }
    }
} 