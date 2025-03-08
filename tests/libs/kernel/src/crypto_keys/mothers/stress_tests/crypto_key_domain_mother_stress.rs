#[cfg(test)]
mod crypto_key_domain_mother_stress_tests {
    use crate::crypto_keys::mothers::crypto_key_domain_mother::CryptoKeyDomainMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let domain = CryptoKeyDomainMother::random();
            let value = domain.value();
            assert!(!value.is_empty());
        }
    }
} 