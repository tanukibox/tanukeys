use std::ops::Deref;

use actix_web::{
    web,
    HttpResponse,
};
use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use kernel::crypto_keys::application::find_one::crypto_key_finder::CryptoKeyFinder;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use kernel::crypto_keys::infrastructure::dtos::crypto_key_json_dto::parse_to_dto;
use kernel::shared::domain::entities::user_id::UserId;

pub(crate) async fn controller<R: CryptoKeyRepository>(
    user_id: web::Path<String>,
    crypto_key_id: web::Path<String>,
    finder: web::Data<CryptoKeyFinder<R>>,
) -> HttpResponse {
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
            if let Some(err) = err.deref().downcast_ref::<DomainError>() {
                match err.get_err_type() {
                    GeneralErrorTypes::ResourceNotFound => {
                        return HttpResponse::NotFound().body(err.message())
                    }
                    GeneralErrorTypes::ResourceAlreadyExists => {
                        return HttpResponse::Conflict().body(err.message())
                    }
                    GeneralErrorTypes::Other => {
                        return HttpResponse::InternalServerError().finish()
                    }
                };
            }
            HttpResponse::InternalServerError().finish()
        }
    }
}
