
use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use cqrs::domain::query_bus::QueryBus;
use serde::Deserialize;
use kernel::crypto_keys::{application::{crypto_key_query_response::CryptoKeyQueryResponse, find_many_by_user::find_crypto_keys_by_user_query::FindCryptoKeysByUserQuery}, domain::crypto_key_repository::CryptoKeyRepository, infrastructure::dtos::crypto_key_json_dto::parse_to_dto};

use crate::v1::errors::parse_domain_error;

#[derive(Debug, Deserialize)]
pub struct Params {
    user_id: String
}

pub(crate) async fn controller<R: CryptoKeyRepository>(
    req: HttpRequest,
    query_bus: web::Data<Arc<dyn QueryBus>>,
) -> HttpResponse {
    let params = web::Query::<Params>::from_query(req.query_string())
        .unwrap_or(web::Query(Params { user_id: String::from("") }));
    let user_id = Some(params.user_id.clone());
    let query = FindCryptoKeysByUserQuery { user_id };

    let res = query_bus.ask(Box::new(query)).await;

    if res.response_type() != "CryptoKeyQueryResponse" {
        return HttpResponse::InternalServerError().finish();
    }

    let res = res.as_any().downcast_ref::<CryptoKeyQueryResponse>().unwrap();
    if res.error.is_some() {
        return parse_domain_error(res.error.as_ref().unwrap())
    }

    if res.data.len() == 0 {
        return HttpResponse::NoContent().body(vec![]);
    }

    let dtos = res.data.iter().map(|crypto_key| parse_to_dto(crypto_key)).collect::<Vec<_>>();
    HttpResponse::Ok().json(dtos)

}
