#[cfg(test)]
mod crypto_key_payload_mother_stress_tests {
    use crate::crypto_keys::mothers::crypto_key_payload_mother::CryptoKeyPayloadMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let payload = CryptoKeyPayloadMother::random();
            assert!(!payload.value().is_empty());
        }
    }
} 