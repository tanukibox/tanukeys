#[cfg(test)]
pub mod crypto_key_domain_tests {
    use kernel::crypto_keys::domain::entities::crypto_key_domain::CryptoKeyDomain;
    use kernel::shared::domain::errors::DomainError;

    #[test]
    fn test_valid_domain() {
        // Test alphanumeric only
        let domain = CryptoKeyDomain::new("testdomain123".to_string()).unwrap();
        assert_eq!(domain.value(), "testdomain123");

        // Test with dots
        let domain = CryptoKeyDomain::new("test.domain.123".to_string()).unwrap();
        assert_eq!(domain.value(), "test.domain.123");

        // Test mixed case
        let domain = CryptoKeyDomain::new("Test.Domain.123".to_string()).unwrap();
        assert_eq!(domain.value(), "Test.Domain.123");
    }

    #[test]
    fn test_single_char_domain() {
        // Test single letter
        let domain = CryptoKeyDomain::new("a".to_string()).unwrap();
        assert_eq!(domain.value(), "a");

        // Test single number
        let domain = CryptoKeyDomain::new("1".to_string()).unwrap();
        assert_eq!(domain.value(), "1");

        // Test single dot
        let domain = CryptoKeyDomain::new(".".to_string()).unwrap();
        assert_eq!(domain.value(), ".");
    }

    #[test]
    fn test_invalid_characters() {
        // Test with space
        let result = CryptoKeyDomain::new("test domain".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
        if let Err(DomainError::ValueObjectError { value }) = result {
            assert_eq!(value, "Crypto key domain must only contain alphanumeric characters and dots");
        }

        // Test with special characters
        let result = CryptoKeyDomain::new("test-domain".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));

        let result = CryptoKeyDomain::new("test_domain".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));

        let result = CryptoKeyDomain::new("test@domain".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_empty_domain() {
        let result = CryptoKeyDomain::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
        if let Err(DomainError::ValueObjectError { value }) = result {
            assert_eq!(value, "Crypto key domain must not be empty");
        }
    }

    #[test]
    fn test_cloning() {
        let domain = CryptoKeyDomain::new("test.domain.123".to_string()).unwrap();
        let cloned = domain.clone();
        assert_eq!(domain, cloned);
    }

    #[test]
    fn test_equality() {
        let domain1 = CryptoKeyDomain::new("test.domain.123".to_string()).unwrap();
        let domain2 = CryptoKeyDomain::new("test.domain.123".to_string()).unwrap();
        assert_eq!(domain1, domain2);
    }
} 