#[cfg(test)]
pub mod crypto_key_name_tests {
    use kernel::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
    use kernel::shared::domain::errors::DomainError;

    #[test]
    fn test_valid_name() {
        let name = CryptoKeyName::new("Test Key".to_string()).unwrap();
        assert_eq!(name.value(), "Test Key");
    }

    #[test]
    fn test_single_char_name() {
        let name = CryptoKeyName::new("a".to_string()).unwrap();
        assert_eq!(name.value(), "a");
    }

    #[test]
    fn test_empty_name() {
        let result = CryptoKeyName::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_cloning() {
        let name = CryptoKeyName::new("Test Key".to_string()).unwrap();
        let cloned = name.clone();
        assert_eq!(name, cloned);
    }

    #[test]
    fn test_equality() {
        let name1 = CryptoKeyName::new("Test Key".to_string()).unwrap();
        let name2 = CryptoKeyName::new("Test Key".to_string()).unwrap();
        assert_eq!(name1, name2);
    }
} 