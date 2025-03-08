#[cfg(test)]
mod crypto_key_description_mother_stress_tests {
    use crate::crypto_keys::mothers::crypto_key_description_mother::CryptoKeyDescriptionMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let description = CryptoKeyDescriptionMother::random();
            let value = description.value();
            assert!(!value.is_empty());
        }
    }
} 