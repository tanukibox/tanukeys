/// Controller for retrieving a specific crypto key.
/// 
/// This module handles GET requests to retrieve a specific cryptographic key by its ID.
/// It requires proper user identification and validates the input parameters.

use actix_web::{web, HttpRequest, HttpResponse};
use kernel::shared::domain::errors::DomainError;
use serde::Deserialize;
use kernel::crypto_keys::application::find_one::crypto_key_finder::CryptoKeyFinder;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use kernel::crypto_keys::infrastructure::dtos::crypto_key_json_dto::parse_to_dto;
use kernel::shared::domain::entities::user_id::UserId;

/// Query parameters for the get crypto key endpoint.
#[derive(Debug, Deserialize)]
pub struct Params {
    /// The ID of the user requesting the crypto key
    user_id: String
}

/// Handles GET requests to retrieve a specific crypto key.
/// 
/// This function processes GET requests to retrieve a cryptographic key by its ID.
/// It validates the user ID and crypto key ID parameters before retrieving the key.
/// 
/// # Arguments
/// 
/// * `crypto_key_id` - The ID of the crypto key to retrieve
/// * `req` - The HTTP request containing query parameters
/// * `finder` - The crypto key finder service
/// 
/// # Returns
/// 
/// An `HttpResponse` with:
/// - 200 OK with the crypto key data on success
/// - 400 Bad Request for invalid parameters
/// - 404 Not Found if the key doesn't exist
/// - 500 Internal Server Error for other errors
pub(crate) async fn controller<R: CryptoKeyRepository>(
    crypto_key_id: web::Path<String>,
    req: HttpRequest,
    finder: web::Data<CryptoKeyFinder<R>>,
) -> HttpResponse {
    let params = web::Query::<Params>::from_query(req.query_string())
        .unwrap_or(web::Query(Params { user_id: String::from("") }));
    let user_id = params.user_id.clone();
    let user_id = UserId::new(user_id.parse().unwrap());
    let crypto_key_id = CryptoKeyId::new(crypto_key_id.parse().unwrap());
    if user_id.is_err() || crypto_key_id.is_err() {
        return HttpResponse::BadRequest().finish()
    }
    let res = finder.run(crypto_key_id.unwrap(), user_id.unwrap()).await;
    match res {
        Ok(key) => {
            let dto = parse_to_dto(&key);
            HttpResponse::Ok().json(dto)
        }
        Err(err) => {
            match err {
                DomainError::CryptoKeyNotFound { id, user_id } => {
                    HttpResponse::NotFound().body(format!("Crypto key with id <{}>, not found for user <{}>.", id, user_id))
                },
                DomainError::CryptoKeyAlreadyExists { id, user_id } => {
                    HttpResponse::Conflict().body(format!("Crypto key with id <{}>, already exists for user <{}>.", id, user_id))
                },
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
}
