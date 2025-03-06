#[cfg(test)]
pub mod crypto_key_status_tests {
    use kernel::crypto_keys::domain::entities::crypto_key_status::CryptoKeyStatus;
    use kernel::shared::domain::errors::DomainError;

    #[test]
    fn test_valid_statuses() {
        let active = CryptoKeyStatus::new("active".to_string()).unwrap();
        assert_eq!(active.value(), "active");
        assert!(active.is_active());
        assert!(!active.is_revoked());

        let revoked = CryptoKeyStatus::new("revoked".to_string()).unwrap();
        assert_eq!(revoked.value(), "revoked");
        assert!(!revoked.is_active());
        assert!(revoked.is_revoked());
    }

    #[test]
    fn test_case_insensitivity() {
        let active_upper = CryptoKeyStatus::new("ACTIVE".to_string()).unwrap();
        assert_eq!(active_upper.value(), "active");
        assert!(active_upper.is_active());

        let revoked_upper = CryptoKeyStatus::new("REVOKED".to_string()).unwrap();
        assert_eq!(revoked_upper.value(), "revoked");
        assert!(revoked_upper.is_revoked());
    }

    #[test]
    fn test_empty_status() {
        let result = CryptoKeyStatus::new("".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_invalid_status() {
        let result = CryptoKeyStatus::new("invalid".to_string());
        assert!(matches!(result, Err(DomainError::ValueObjectError { .. })));
    }

    #[test]
    fn test_cloning() {
        let status = CryptoKeyStatus::new("active".to_string()).unwrap();
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_equality() {
        let status1 = CryptoKeyStatus::new("active".to_string()).unwrap();
        let status2 = CryptoKeyStatus::new("active".to_string()).unwrap();
        assert_eq!(status1, status2);
    }
} 