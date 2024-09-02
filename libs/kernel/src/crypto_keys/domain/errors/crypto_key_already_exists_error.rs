use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::errors::crypto_key_errors::CryptoKeyErrorTypes;
use crate::shared::domain::entities::user_id::UserId;


pub fn crypto_key_already_exists_error(user_id: UserId, key_id: CryptoKeyId) -> DomainError {
    let err_msg = format!("CryptoKey with id {} and user {} already exists.", key_id.value(), user_id.value());
    let err_type = GeneralErrorTypes::ResourceAlreadyExists;
    let spec_err_type = CryptoKeyErrorTypes::CryptoKeyAlreadyExists;
    DomainError::new(err_msg, err_type, spec_err_type.to_string())
}
