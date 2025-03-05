/// Controller for retrieving all crypto keys for a user.
/// 
/// This module handles GET requests to retrieve all cryptographic keys associated
/// with a specific user. It requires proper user identification.

use actix_web::{web, HttpRequest, HttpResponse};
use kernel::crypto_keys::application::find_many_by_user::crypto_keys_by_user_finder::CryptoKeysByUserFinder;
use serde::Deserialize;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::infrastructure::dtos::crypto_key_json_dto::parse_to_dto;
use kernel::shared::domain::entities::user_id::UserId;

/// Query parameters for the get all crypto keys endpoint.
#[derive(Debug, Deserialize)]
pub struct Params {
    /// The ID of the user whose crypto keys to retrieve
    user_id: String
}

/// Handles GET requests to retrieve all crypto keys for a user.
/// 
/// This function processes GET requests to retrieve all cryptographic keys
/// associated with a specific user. It validates the user ID parameter.
/// 
/// # Arguments
/// 
/// * `req` - The HTTP request containing query parameters
/// * `finder` - The crypto keys finder service
/// 
/// # Returns
/// 
/// An `HttpResponse` with:
/// - 200 OK with the list of crypto keys on success
/// - 400 Bad Request for invalid parameters
/// - 500 Internal Server Error for other errors
pub(crate) async fn controller<R: CryptoKeyRepository>(
    req: HttpRequest,
    finder: web::Data<CryptoKeysByUserFinder<R>>,
) -> HttpResponse {
    let params = web::Query::<Params>::from_query(req.query_string())
        .unwrap_or(web::Query(Params { user_id: String::from("") }));
    let user_id = params.user_id.clone();
    let user_id = UserId::new(user_id.parse().unwrap());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish()
    }

    let res = finder.run(user_id.unwrap()).await;
    match res {
        Ok(keys) => {
            let dtos: Vec<_> = keys.iter().map(|key| parse_to_dto(&key)).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(err) => {
            match err {
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
}
