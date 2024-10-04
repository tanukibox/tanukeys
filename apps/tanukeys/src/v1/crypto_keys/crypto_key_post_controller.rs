
use actix_web::{web, HttpRequest, HttpResponse};
use events::domain::event_bus::EventBus;
use kernel::crypto_keys::application::create_one::crypto_key_creator::CryptoKeyCreator;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::infrastructure::dtos::crypto_key_json_dto::{parse_to_domain, CryptoKeyJsonDto};
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;

pub async fn controller<R: CryptoKeyRepository, E: EventBus>(
    dto: web::Json<CryptoKeyJsonDto>,
    req: HttpRequest,
    creator: web::Data<CryptoKeyCreator<R, E>>,
) -> HttpResponse {

    let auth_user = req.headers().get("Authorization");
    if auth_user.is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let auth_user = UserId::new(Some(auth_user.unwrap().to_str().unwrap_or("").to_string()));
    if auth_user.is_err() {
        return HttpResponse::Unauthorized().finish();
    }
    let auth_user = auth_user.unwrap();

    let key = parse_to_domain(&dto);
    if key.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let key = key.unwrap();

    let res = creator.run(key.id, key.name, key.payload, key.user_id, key.description, auth_user).await;
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
