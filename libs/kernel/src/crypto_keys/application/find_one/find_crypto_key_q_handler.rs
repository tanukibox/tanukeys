use std::ops::Deref;

use async_trait::async_trait;
use cqrs::domain::{query::Query, query_bus_response::QueryBusResponse, query_handler::QueryHandler};

use crate::{crypto_keys::{application::crypto_key_query_response::CryptoKeyQueryResponse, domain::{crypto_key_repository::CryptoKeyRepository, entities::crypto_key_id::CryptoKeyId}}, shared::domain::entities::user_id::UserId};

use super::{crypto_key_finder::CryptoKeyFinder, find_crypto_key_query::FindCryptoKeyQuery};




pub struct FindCryptoKeyQueryHandler<R: CryptoKeyRepository> {
    crypto_keys_by_user_finder: CryptoKeyFinder<R>,
}

impl<R: CryptoKeyRepository> FindCryptoKeyQueryHandler<R> {
    pub fn new(crypto_keys_by_user_finder: CryptoKeyFinder<R>) -> FindCryptoKeyQueryHandler<R> {
        FindCryptoKeyQueryHandler { crypto_keys_by_user_finder }
    }
}

#[async_trait]
impl <R: CryptoKeyRepository> QueryHandler for FindCryptoKeyQueryHandler<R> {
    async fn handle(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse> {
        let query = query.deref().as_any().downcast_ref::<FindCryptoKeyQuery>().unwrap();
        let user_id_build = UserId::new(query.user_id.clone());
        let crypto_key_id_build = CryptoKeyId::new(query.crypto_key_id.clone());
        if user_id_build.is_err() {
            return CryptoKeyQueryResponse::boxed_err(user_id_build.unwrap_err());
        }
        if crypto_key_id_build.is_err() {
            return CryptoKeyQueryResponse::boxed_err(crypto_key_id_build.unwrap_err());
        }
        let user_id = user_id_build.unwrap();
        let crypto_key_id = crypto_key_id_build.unwrap();
        let res = self.crypto_keys_by_user_finder.run(crypto_key_id, user_id).await;
        if res.is_err() {
            return CryptoKeyQueryResponse::boxed_err(res.unwrap_err());
        }
        
        CryptoKeyQueryResponse::boxed_entities(vec![res.unwrap()])
    }

    fn subscribet_to(&self) -> String {
        return FindCryptoKeyQuery::QUERY_TYPE.to_string();
    }
}