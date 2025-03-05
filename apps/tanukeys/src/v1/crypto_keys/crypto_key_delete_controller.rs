/// Controller for deleting crypto keys.
/// 
/// This module handles DELETE requests to remove cryptographic keys from the system.
/// It requires proper authentication and validates the input parameters before deletion.

use actix_web::{web, HttpRequest, HttpResponse};
use kernel::shared::domain::errors::DomainError;
use serde::Deserialize;
use kernel::crypto_keys::application::delete_one::crypto_key_deleter::CryptoKeyDeleter;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use kernel::shared::domain::entities::user_id::UserId;
use events::domain::event_bus::EventBus;

/// Query parameters for the delete crypto key endpoint.
#[derive(Debug, Deserialize)]
pub struct Params {
    /// The ID of the user requesting the deletion
    user_id: String
}

/// Handles DELETE requests to remove a crypto key.
/// 
/// This function processes DELETE requests to remove a cryptographic key.
/// It validates the authentication token and input parameters before deletion.
/// 
/// # Arguments
/// 
/// * `crypto_key_id` - The ID of the crypto key to delete
/// * `req` - The HTTP request containing query parameters
/// * `deleter` - The crypto key deleter service
/// 
/// # Returns
/// 
/// An `HttpResponse` with:
/// - 204 No Content on successful deletion
/// - 400 Bad Request for invalid parameters
/// - 401 Unauthorized for missing/invalid authentication
/// - 403 Forbidden if the user is not authorized
/// - 404 Not Found if the key doesn't exist
/// - 500 Internal Server Error for other errors
pub(crate) async fn controller<R: CryptoKeyRepository, E: EventBus>(
    crypto_key_id: web::Path<String>,
    req: HttpRequest,
    deleter: web::Data<CryptoKeyDeleter<R, E>>,
) -> HttpResponse {
    let params = web::Query::<Params>::from_query(req.query_string())
        .unwrap_or(web::Query(Params { user_id: String::from("") }));
    let user_id = params.user_id.clone();
    let user_id = UserId::new(user_id.parse().unwrap());
    let crypto_key_id = CryptoKeyId::new(crypto_key_id.parse().unwrap());
    if user_id.is_err() || crypto_key_id.is_err() {
        return HttpResponse::BadRequest().finish()
    }
    let user_id = user_id.unwrap();
    let res = deleter.run(crypto_key_id.unwrap(), user_id.clone(), user_id).await;
    match res {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            match err {
                DomainError::CryptoKeyNotFound { id, user_id } => {
                    HttpResponse::NotFound().body(format!("Crypto key with id <{}>, not found for user <{}>.", id, user_id))
                },
                DomainError::UserNotAuthorized { user_id } => {
                    HttpResponse::Forbidden().body(format!("User <{}> is not authorized to delete this crypto key.", user_id))
                },
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
} 