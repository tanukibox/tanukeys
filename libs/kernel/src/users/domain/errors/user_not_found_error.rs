use domain_errors::domain_error::{DomainError, GeneralErrorTypes};

use crate::users::domain::entities::user_id::UserId;

use super::user_errors::UserErrorTypes;

pub fn user_not_found_error(user_id: UserId) -> DomainError {
    let err_msg = format!("User with id {} not found.", user_id.value());
    let err_type = GeneralErrorTypes::ResourceNotFound;
    let spec_err_type = UserErrorTypes::UserNotFound;
    DomainError::new(err_msg, err_type, spec_err_type.to_string())
}
