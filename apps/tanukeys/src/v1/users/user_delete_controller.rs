/// Controller for deleting users.
/// 
/// This module handles DELETE requests to remove users from the system.
/// It requires proper authentication and validates the input parameters before deletion.

use actix_web::{
    web, HttpRequest, HttpResponse
};
use events::domain::event_bus::EventBus;
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;
use kernel::users::{application::delete_one::user_deleter::UserDeleter, domain::user_repository::UserRepository};

/// Handles DELETE requests to remove a user.
/// 
/// This function processes DELETE requests to remove a user.
/// It validates the authentication token and input parameters before deletion.
/// 
/// # Arguments
/// 
/// * `user_id` - The ID of the user to delete
/// * `req` - The HTTP request containing authentication headers
/// * `deleter` - The user deleter service
/// 
/// # Returns
/// 
/// An `HttpResponse` with:
/// - 202 Accepted on successful deletion
/// - 400 Bad Request for invalid parameters
/// - 401 Unauthorized for missing/invalid authentication
/// - 404 Not Found if the user doesn't exist
/// - 500 Internal Server Error for other errors
pub(crate) async fn controller<R: UserRepository, E: EventBus>(
    user_id: web::Path<String>,
    req: HttpRequest,
    deleter: web::Data<UserDeleter<R, E>>,
) -> HttpResponse {
    let auth_user = req.headers().get("Authorization");
    if auth_user.is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let auth_user = UserId::new(auth_user.unwrap().to_str().unwrap().to_string());
    if auth_user.is_err() {
        return HttpResponse::Unauthorized().finish();
    }
    let auth_user = auth_user.unwrap();
    let user_id = UserId::new(user_id.clone());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let user_id = user_id.unwrap();

    let res = deleter.run(user_id, auth_user).await;
    match res {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(err) => {
            match err {
                DomainError::UserAlreadyExists { user_id } => {
                    return HttpResponse::Conflict().body(format!("User with id <{}>, already exists.", user_id))
                },
                DomainError::UserNotFound { user_id } => {
                    return HttpResponse::NotFound().body(format!("User with id <{}>, not found.", user_id))
                },
                DomainError::UserNotAuthorized { user_id } => {
                    return HttpResponse::Unauthorized().body(format!("User with id <{}>, not authorized.", user_id))
                },
                _ => return HttpResponse::InternalServerError().finish()
            }
        }
    }
}
