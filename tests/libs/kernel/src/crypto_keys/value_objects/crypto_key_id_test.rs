#[cfg(test)]
pub mod crypto_key_id_tests {
    use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
    use kernel::shared::domain::errors::DomainError;

    #[test]
    fn test_valid_id() {
        let id = CryptoKeyId::new("test-id-123".to_string()).unwrap();
        assert_eq!(id.value(), "test-id-123");
    }

    #[test]
    fn test_single_char_id() {
        let id = CryptoKeyId::new("a".to_string()).unwrap();
        assert_eq!(id.value(), "a");
    }

    #[test]
    fn test_empty_id() {
        let result = CryptoKeyId::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_cloning() {
        let id = CryptoKeyId::new("test-id-123".to_string()).unwrap();
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_equality() {
        let id1 = CryptoKeyId::new("test-id-123".to_string()).unwrap();
        let id2 = CryptoKeyId::new("test-id-123".to_string()).unwrap();
        assert_eq!(id1, id2);
    }
} 