#[cfg(test)]
mod crypto_key_status_mother_stress_tests {
    use crate::crypto_keys::mothers::crypto_key_status_mother::CryptoKeyStatusMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let status = CryptoKeyStatusMother::random();
            let value = status.value();
            assert!(matches!(value.as_str(), "active" | "revoked" | "chain_broken"));
        }
    }
} 