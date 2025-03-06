#[cfg(test)]
pub mod crypto_key_domain_tests {
    use kernel::crypto_keys::domain::entities::crypto_key_domain::CryptoKeyDomain;
    use kernel::shared::domain::errors::DomainError;

    #[test]
    fn test_valid_domain() {
        let domain = CryptoKeyDomain::new("test-domain".to_string()).unwrap();
        assert_eq!(domain.value(), "test-domain");
    }

    #[test]
    fn test_empty_domain() {
        let result = CryptoKeyDomain::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_cloning() {
        let domain = CryptoKeyDomain::new("test-domain".to_string()).unwrap();
        let cloned = domain.clone();
        assert_eq!(domain, cloned);
    }

    #[test]
    fn test_equality() {
        let domain1 = CryptoKeyDomain::new("test-domain".to_string()).unwrap();
        let domain2 = CryptoKeyDomain::new("test-domain".to_string()).unwrap();
        assert_eq!(domain1, domain2);
    }
} 