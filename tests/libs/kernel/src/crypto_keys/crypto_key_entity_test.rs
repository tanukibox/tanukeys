#[cfg(test)]
pub mod crypto_key_tests {
    use crate::crypto_keys::mothers::crypto_key_mother::CryptoKeyMother;
    use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
    use kernel::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
    use kernel::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
    use kernel::crypto_keys::domain::entities::crypto_key_description::CryptoKeyDescription;
    use kernel::shared::domain::entities::user_id::UserId;

    #[test]
    fn create_simple_key() {
        let key = CryptoKeyMother::random();
        assert!(key.id.value().len() > 0);
        assert!(key.name.value().len() > 0);
        assert!(key.payload.value().len() > 0);
        assert!(key.user_id.value().len() > 0);
        assert!(key.description.value().len() > 0);
    }

    #[test]
    fn create_key_with_specific_values() {
        let id = "test-id-123".to_string();
        let name = "Test Key".to_string();
        let payload = "test-payload".to_string();
        let user_id = "user-123".to_string();
        let description = "Test Description".to_string();

        let key = CryptoKeyMother::with_params(
            Some(id.clone()),
            Some(name.clone()),
            Some(description.clone()),
            Some(payload.clone()),
            Some(user_id.clone()),
        );

        assert_eq!(key.id.value(), id);
        assert_eq!(key.name.value(), name);
        assert_eq!(key.payload.value(), payload);
        assert_eq!(key.user_id.value(), user_id);
        assert_eq!(key.description.value(), description);
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
    }

    #[test]
    fn test_key_equality() {
        let id = "test-id-123".to_string();
        let name = "Test Key".to_string();
        let payload = "test-payload".to_string();
        let user_id = "user-123".to_string();
        let description = "Test Description".to_string();

        let key1 = CryptoKeyMother::with_params(
            Some(id.clone()),
            Some(name.clone()),
            Some(description.clone()),
            Some(payload.clone()),
            Some(user_id.clone()),
        );

        let key2 = CryptoKeyMother::with_params(
            Some(id),
            Some(name),
            Some(description),
            Some(payload),
            Some(user_id),
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
    }

}