use domain_errors::domain_error::{DomainError, GeneralErrorTypes};

use crate::shared::domain::entities::user_id::UserId;

use super::user_errors::UserErrorTypes;

pub fn user_already_exists_error(user_id: UserId) -> DomainError {
    let err_msg = format!("User with id {} already exists.", user_id.value());
    let err_type = GeneralErrorTypes::ResourceAlreadyExists;
    let spec_err_type = UserErrorTypes::UserAlreadyExists;
    DomainError::new(err_msg, err_type, spec_err_type.to_string())
}
