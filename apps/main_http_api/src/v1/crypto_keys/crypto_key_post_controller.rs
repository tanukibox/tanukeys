use std::ops::Deref;

use actix_web::{web, HttpRequest, HttpResponse};
use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use events::domain::event_bus::EventBus;
use kernel::crypto_keys::application::create_one::crypto_key_creator::CryptoKeyCreator;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::infrastructure::dtos::crypto_key_json_dto::{parse_to_domain, CryptoKeyJsonDto};
use kernel::shared::domain::entities::user_id::UserId;

pub async fn controller<R: CryptoKeyRepository, E: EventBus>(
    dto: web::Json<CryptoKeyJsonDto>,
    req: HttpRequest,
    creator: web::Data<CryptoKeyCreator<R, E>>,
) -> HttpResponse {
    let auth_user = req.headers().get("X-USER");
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

    let res = creator.run(key.id, key.name, key.payload, key.user_id, auth_user).await;
    match res {
        Ok(_) => HttpResponse::Accepted().finish(),
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
