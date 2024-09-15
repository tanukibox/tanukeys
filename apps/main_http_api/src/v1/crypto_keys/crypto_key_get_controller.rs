
use actix_web::{web, HttpRequest, HttpResponse};
use kernel::crypto_keys::application::find_many_by_user::crypto_keys_by_user_finder::CryptoKeysByUserFinder;
use serde::Deserialize;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::crypto_keys::infrastructure::dtos::crypto_key_json_dto::parse_to_dto;
use kernel::shared::domain::entities::user_id::UserId;

#[derive(Debug, Deserialize)]
pub struct Params {
    user_id: String
}

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
