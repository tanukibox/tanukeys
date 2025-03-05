#[cfg(test)]
pub mod crypto_key_tests {
    use crate::crypto_keys::mothers::crypto_key_mother::CryptoKeyMother;
    use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
    use kernel::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
    use kernel::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
    use kernel::crypto_keys::domain::entities::crypto_key_description::CryptoKeyDescription;
    use kernel::crypto_keys::domain::entities::crypto_key_type::CryptoKeyType;
    use kernel::crypto_keys::domain::entities::crypto_key_domain::CryptoKeyDomain;
    use kernel::crypto_keys::domain::entities::crypto_key_status::CryptoKeyStatus;
    use kernel::shared::domain::entities::user_id::UserId;

    #[test]
    fn create_simple_key() {
        let key = CryptoKeyMother::random();
        assert!(key.id.value().len() > 0);
        assert!(key.name.value().len() > 0);
        assert!(key.payload.value().len() > 0);
        assert!(key.user_id.value().len() > 0);
        assert!(key.description.value().len() > 0);
        assert!(key.key_type.value().len() > 0);
        assert!(key.domain.value().len() > 0);
        assert!(key.status.value().len() > 0);
    }

    #[test]
    fn create_key_with_specific_values() {
        let id = "test-id-123".to_string();
        let name = "Test Key".to_string();
        let payload = "test-payload".to_string();
        let user_id = "user-123".to_string();
        let description = "Test Description".to_string();
        let key_type = "test-type".to_string();
        let domain = "test-domain".to_string();
        let status = "active".to_string();

        let key = CryptoKeyMother::with_params(
            Some(id.clone()),
            Some(name.clone()),
            Some(description.clone()),
            Some(payload.clone()),
            Some(user_id.clone()),
            Some(key_type.clone()),
            Some(domain.clone()),
            Some(status.clone()),
        );

        assert_eq!(key.id.value(), id);
        assert_eq!(key.name.value(), name);
        assert_eq!(key.payload.value(), payload);
        assert_eq!(key.user_id.value(), user_id);
        assert_eq!(key.description.value(), description);
        assert_eq!(key.key_type.value(), key_type);
        assert_eq!(key.domain.value(), domain);
        assert_eq!(key.status.value(), status);
    }

    #[test]
    fn test_key_cloning() {
        let original = CryptoKeyMother::random();
        let cloned = original.clone();

        assert_eq!(original.id.value(), cloned.id.value());
        assert_eq!(original.name.value(), cloned.name.value());
        assert_eq!(original.payload.value(), cloned.payload.value());
        assert_eq!(original.user_id.value(), cloned.user_id.value());
        assert_eq!(original.description.value(), cloned.description.value());
        assert_eq!(original.key_type.value(), cloned.key_type.value());
        assert_eq!(original.domain.value(), cloned.domain.value());
        assert_eq!(original.status.value(), cloned.status.value());
    }

    #[test]
    fn test_key_equality() {
        let id = "test-id-123".to_string();
        let name = "Test Key".to_string();
        let payload = "test-payload".to_string();
        let user_id = "user-123".to_string();
        let description = "Test Description".to_string();
        let key_type = "test-type".to_string();
        let domain = "test-domain".to_string();
        let status = "active".to_string();

        let key1 = CryptoKeyMother::with_params(
            Some(id.clone()),
            Some(name.clone()),
            Some(description.clone()),
            Some(payload.clone()),
            Some(user_id.clone()),
            Some(key_type.clone()),
            Some(domain.clone()),
            Some(status.clone()),
        );

        let key2 = CryptoKeyMother::with_params(
            Some(id),
            Some(name),
            Some(description),
            Some(payload),
            Some(user_id),
            Some(key_type),
            Some(domain),
            Some(status),
        );

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_key_inequality() {
        let key1 = CryptoKeyMother::random();
        let key2 = CryptoKeyMother::random();

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_key_validation() {
        // Test with empty values
        let result = CryptoKeyId::new("".to_string());
        assert!(result.is_err());

        let result = CryptoKeyName::new("".to_string());
        assert!(result.is_err());

        let result = CryptoKeyPayload::new("".to_string());
        assert!(result.is_err());

        let result = UserId::new("".to_string());
        assert!(result.is_err());

        let result = CryptoKeyDescription::new("".to_string());
        assert!(result.is_err());

        let result = CryptoKeyType::new("".to_string());
        assert!(result.is_err());

        let result = CryptoKeyDomain::new("".to_string());
        assert!(result.is_err());

        let result = CryptoKeyStatus::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_crypto_key_status_validation() {
        // Test valid statuses
        let active = CryptoKeyStatus::new("active".to_string()).unwrap();
        assert_eq!(active.value(), "active");
        assert!(active.is_active());
        assert!(!active.is_revoked());

        let revoked = CryptoKeyStatus::new("revoked".to_string()).unwrap();
        assert_eq!(revoked.value(), "revoked");
        assert!(!revoked.is_active());
        assert!(revoked.is_revoked());

        // Test case insensitivity
        let active_upper = CryptoKeyStatus::new("ACTIVE".to_string()).unwrap();
        assert_eq!(active_upper.value(), "active");
        assert!(active_upper.is_active());

        // Test invalid status
        let result = CryptoKeyStatus::new("invalid".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid crypto key status"));
    }
}