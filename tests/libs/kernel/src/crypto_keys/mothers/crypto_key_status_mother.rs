use kernel::crypto_keys::domain::entities::crypto_key_status::CryptoKeyStatus;
use rand::Rng;

/// A factory for creating test instances of `CryptoKeyStatus`.
/// 
/// This mother class provides methods to generate random or specific instances of `CryptoKeyStatus`
/// for testing purposes. It ensures that all generated statuses are valid according to the domain rules.
pub struct CryptoKeyStatusMother {}

impl CryptoKeyStatusMother {
    /// Generates a random valid crypto key status.
    /// 
    /// This method randomly selects between "active" and "revoked" statuses with equal probability.
    /// The generated status is guaranteed to be valid according to the domain rules.
    /// 
    /// # Returns
    /// 
    /// A new `CryptoKeyStatus` instance with a random valid status.
    pub fn random() -> CryptoKeyStatus {
        let mut rng = rand::thread_rng();
        let status = if rng.gen_bool(0.5) {
            "active"
        } else {
            "revoked"
        };
        CryptoKeyStatus::new(status.to_string()).unwrap()
    }

    /// Creates a crypto key status with specific parameters.
    /// 
    /// This method allows creating a `CryptoKeyStatus` with a specific value, or generates a random
    /// one if no value is provided.
    /// 
    /// # Arguments
    /// 
    /// * `value` - An optional string value for the status. If None, a random valid status is generated.
    /// 
    /// # Returns
    /// 
    /// A new `CryptoKeyStatus` instance with the specified value or a random valid status.
    /// 
    /// # Panics
    /// 
    /// Panics if the provided value is not a valid status according to the domain rules.
    pub fn with_params(value: Option<String>) -> CryptoKeyStatus {
        match value {
            Some(value) => CryptoKeyStatus::new(value).unwrap(),
            None => Self::random(),
        }
    }
} 