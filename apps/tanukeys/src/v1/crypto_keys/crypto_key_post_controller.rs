/// Controller for creating new crypto keys.
/// 
/// This module handles POST requests to create new cryptographic keys in the system.
/// It requires proper authentication and validates the input data before creating the key.

use actix_web::{web, HttpRequest, HttpResponse};
use events::domain::event_bus::EventBus;
use kernel::crypto_keys::application::create_one::crypto_key_creator::CryptoKeyCreator;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::infrastructure::dtos::crypto_key_json_dto::{parse_to_domain, CryptoKeyJsonDto};
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;

/// Handles POST requests to create a new crypto key.
/// 
/// This function processes POST requests to create a new cryptographic key.
/// It validates the authentication token and input data before creating the key.
/// 
/// # Arguments
/// 
/// * `dto` - The JSON DTO containing the crypto key data
/// * `req` - The HTTP request containing authentication headers
/// * `creator` - The crypto key creator service
/// 
/// # Returns
/// 
/// An `HttpResponse` with:
/// - 202 Accepted on successful creation
/// - 400 Bad Request for invalid input
/// - 401 Unauthorized for missing/invalid authentication
/// - 409 Conflict if the key already exists
/// - 500 Internal Server Error for other errors
pub async fn controller<R: CryptoKeyRepository, E: EventBus>(
    dto: web::Json<CryptoKeyJsonDto>,
    req: HttpRequest,
    creator: web::Data<CryptoKeyCreator<R, E>>,
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

    let key = parse_to_domain(&dto);
    if key.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let key = key.unwrap();

    let res = creator.run(
        key.id,
        key.name,
        key.payload,
        key.user_id,
        key.description,
        key.key_type,
        key.domain,
        key.status,
        auth_user,
    ).await;
    match res {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(err) => {
            match err {
                DomainError::CryptoKeyNotFound { id, user_id } => {
                    HttpResponse::NotFound().body(format!("Crypto key with id <{}>, not found for user <{}>.", id, user_id))
                },
                DomainError::CryptoKeyAlreadyExists { id, user_id } => {
                    HttpResponse::Conflict().body(format!("Crypto key with id <{}>, already exists for user <{}>.", id, user_id))
                },
                DomainError::UserNotAuthorized { user_id } => {
                    HttpResponse::Unauthorized().body(format!("User <{}> not authorized.", user_id))
                },
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
}
