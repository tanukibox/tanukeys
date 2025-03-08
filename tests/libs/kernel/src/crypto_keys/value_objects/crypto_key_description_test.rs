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
    fn test_single_char_description() {
        let description = CryptoKeyDescription::new("a".to_string()).unwrap();
        assert_eq!(description.value(), "a");
    }

    #[test]
    fn test_description_with_single_spaces() {
        let description = CryptoKeyDescription::new("This is a valid description".to_string()).unwrap();
        assert_eq!(description.value(), "This is a valid description");
    }

    #[test]
    fn test_empty_description() {
        let result = CryptoKeyDescription::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_description_starting_with_space() {
        let result = CryptoKeyDescription::new(" starts with space".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
        if let Err(DomainError::ValueObjectError { value }) = result {
            assert_eq!(value, "Crypto key description must not start with a space");
        }
    }

    #[test]
    fn test_description_ending_with_space() {
        let result = CryptoKeyDescription::new("ends with space ".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
        if let Err(DomainError::ValueObjectError { value }) = result {
            assert_eq!(value, "Crypto key description must not end with a space");
        }
    }

    #[test]
    fn test_description_with_consecutive_spaces() {
        let result = CryptoKeyDescription::new("contains  two spaces".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
        if let Err(DomainError::ValueObjectError { value }) = result {
            assert_eq!(value, "Crypto key description must not contain consecutive spaces");
        }

        let result = CryptoKeyDescription::new("contains   three spaces".to_string());
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