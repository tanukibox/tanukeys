
use actix_web::{web, HttpRequest, HttpResponse};
use kernel::shared::domain::errors::DomainError;
use serde::Deserialize;
use kernel::crypto_keys::application::find_one::crypto_key_finder::CryptoKeyFinder;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use kernel::crypto_keys::infrastructure::dtos::crypto_key_json_dto::parse_to_dto;
use kernel::shared::domain::entities::user_id::UserId;

#[derive(Debug, Deserialize)]
pub struct Params {
    user_id: String
}

pub(crate) async fn controller<R: CryptoKeyRepository>(
    crypto_key_id: web::Path<String>,
    req: HttpRequest,
    finder: web::Data<CryptoKeyFinder<R>>,
) -> HttpResponse {
    let params = web::Query::<Params>::from_query(req.query_string())
        .unwrap_or(web::Query(Params { user_id: String::from("") }));
    let user_id = Some(params.user_id.clone());
    let user_id = UserId::new(user_id);
    let crypto_key_id = CryptoKeyId::new(Some(crypto_key_id.parse().unwrap()));
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
