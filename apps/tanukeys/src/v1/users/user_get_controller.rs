/// Controller for retrieving a specific user.
/// 
/// This module handles GET requests to retrieve a specific user by their ID.
/// It requires proper user identification and validates the input parameters.

use actix_web::{
    web,
    HttpResponse,
};
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;
use kernel::users::{
    application::find_one::user_finder::UserFinder,
    domain::user_repository::UserRepository,
    infrastructure::dtos::json::user_dto::parse_to_dto,
};

/// Handles GET requests to retrieve a specific user.
/// 
/// This function processes GET requests to retrieve a user by their ID.
/// It validates the user ID parameter before retrieving the user.
/// 
/// # Arguments
/// 
/// * `user_id` - The ID of the user to retrieve
/// * `finder` - The user finder service
/// 
/// # Returns
/// 
/// An `HttpResponse` with:
/// - 200 OK with the user data on success
/// - 400 Bad Request for invalid parameters
/// - 404 Not Found if the user doesn't exist
/// - 500 Internal Server Error for other errors
pub(crate) async fn controller<R: UserRepository>(
    user_id: web::Path<String>,
    finder: web::Data<UserFinder<R>>,
) -> HttpResponse {
    let user_id = UserId::new(user_id.parse().unwrap());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish()
    }
    let res = finder.run(user_id.unwrap()).await;
    match res {
        Ok(user) => {
            let dto = parse_to_dto(&user);
            HttpResponse::Ok().json(dto)
        }
        Err(err) => {
            match err {
                DomainError::UserAlreadyExists { user_id } => {
                    return HttpResponse::Conflict().body(format!("User with id <{}>, already exists.", user_id))
                },
                DomainError::UserNotFound { user_id } => {
                    return HttpResponse::NotFound().body(format!("User with id <{}>, not found.", user_id))
                },
                _ => return HttpResponse::InternalServerError().finish()
            }
        }
    }
}
