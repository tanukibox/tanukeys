use actix_web::{web, HttpRequest, HttpResponse};
use kernel::shared::domain::errors::DomainError;
use serde::Deserialize;
use kernel::crypto_keys::application::delete_one::crypto_key_deleter::CryptoKeyDeleter;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use kernel::shared::domain::entities::user_id::UserId;
use events::domain::event_bus::EventBus;

#[derive(Debug, Deserialize)]
pub struct Params {
    user_id: String
}

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