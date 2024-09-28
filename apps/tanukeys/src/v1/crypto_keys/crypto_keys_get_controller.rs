
use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use cqrs::domain::query_bus::QueryBus;
use kernel::crypto_keys::application::find_many_by_user::find_crypto_keys_by_user_query::FindCryptoKeysByUserQuery;
use serde::Deserialize;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;

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
    let user_id = params.user_id.clone();
    let query = FindCryptoKeysByUserQuery { user_id };

    let _res = query_bus.ask(Box::new(query)).await;

    todo!()
}
