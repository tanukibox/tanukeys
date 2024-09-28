use async_trait::async_trait;
use cqrs::domain::{query::Query, query_bus_response::QueryBusResponse, query_handler::QueryHandler};

use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;

use super::{crypto_keys_by_user_finder::CryptoKeysByUserFinder, find_crypto_keys_by_user_query::FindCryptoKeysByUserQuery};


pub struct FindCryptoKeysByUserQueryHandler<R: CryptoKeyRepository> {
    crypto_keys_by_user_finder: CryptoKeysByUserFinder<R>,
}

impl<R: CryptoKeyRepository> FindCryptoKeysByUserQueryHandler<R> {
    pub fn new(crypto_keys_by_user_finder: CryptoKeysByUserFinder<R>) -> FindCryptoKeysByUserQueryHandler<R> {
        FindCryptoKeysByUserQueryHandler { crypto_keys_by_user_finder }
    }
}

#[async_trait]
impl <R: CryptoKeyRepository> QueryHandler for FindCryptoKeysByUserQueryHandler<R> {
    async fn handle(&self, _query: Box<dyn Query>) -> Box<dyn QueryBusResponse> {
        todo!()
    }

    fn subscribet_to(&self) -> String {
        return FindCryptoKeysByUserQuery::QUERY_TYPE.to_string();
    }
}