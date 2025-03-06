#[cfg(test)]
pub mod crypto_key_description_tests {
    use kernel::crypto_keys::domain::entities::crypto_key_description::CryptoKeyDescription;
    use kernel::shared::domain::errors::DomainError;

    #[test]
    fn test_valid_description() {
        let description = CryptoKeyDescription::new("Test Description".to_string()).unwrap();
        assert_eq!(description.value(), "Test Description");
    }

    #[test]
    fn test_empty_description() {
        let result = CryptoKeyDescription::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_cloning() {
        let description = CryptoKeyDescription::new("Test Description".to_string()).unwrap();
        let cloned = description.clone();
        assert_eq!(description, cloned);
    }

    #[test]
    fn test_equality() {
        let description1 = CryptoKeyDescription::new("Test Description".to_string()).unwrap();
        let description2 = CryptoKeyDescription::new("Test Description".to_string()).unwrap();
        assert_eq!(description1, description2);
    }
} 