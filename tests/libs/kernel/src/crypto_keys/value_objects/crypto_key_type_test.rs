#[cfg(test)]
pub mod crypto_key_type_tests {
    use kernel::crypto_keys::domain::entities::crypto_key_type::CryptoKeyType;
    use kernel::shared::domain::errors::DomainError;

    #[test]
    fn test_valid_type() {
        let key_type = CryptoKeyType::new("test-type".to_string()).unwrap();
        assert_eq!(key_type.value(), "test-type");
    }

    #[test]
    fn test_empty_type() {
        let result = CryptoKeyType::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_cloning() {
        let key_type = CryptoKeyType::new("test-type".to_string()).unwrap();
        let cloned = key_type.clone();
        assert_eq!(key_type, cloned);
    }

    #[test]
    fn test_equality() {
        let key_type1 = CryptoKeyType::new("test-type".to_string()).unwrap();
        let key_type2 = CryptoKeyType::new("test-type".to_string()).unwrap();
        assert_eq!(key_type1, key_type2);
    }
} 