#[cfg(test)]
pub mod crypto_key_payload_tests {
    use kernel::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
    use kernel::shared::domain::errors::DomainError;

    #[test]
    fn test_valid_payload() {
        let payload = CryptoKeyPayload::new("test-payload".to_string()).unwrap();
        assert_eq!(payload.value(), "test-payload");
    }

    #[test]
    fn test_empty_payload() {
        let result = CryptoKeyPayload::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_cloning() {
        let payload = CryptoKeyPayload::new("test-payload".to_string()).unwrap();
        let cloned = payload.clone();
        assert_eq!(payload, cloned);
    }

    #[test]
    fn test_equality() {
        let payload1 = CryptoKeyPayload::new("test-payload".to_string()).unwrap();
        let payload2 = CryptoKeyPayload::new("test-payload".to_string()).unwrap();
        assert_eq!(payload1, payload2);
    }
} 