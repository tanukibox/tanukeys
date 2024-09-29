use std::ops::Deref;

use async_trait::async_trait;
use cqrs::domain::{query::Query, query_bus_response::QueryBusResponse, query_handler::QueryHandler};

use crate::{crypto_keys::domain::crypto_key_repository::CryptoKeyRepository, shared::domain::{entities::user_id::UserId, errors::DomainError}, users::domain::entities::user};

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
    async fn handle(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse> {
        let query = query.deref().as_any().downcast_ref::<FindCryptoKeysByUserQuery>().unwrap();
        let user_id_build = UserId::new(query.user_id.clone());
        if user_id_build.is_err() {
            panic!("Error");
        }
        let user_id = user_id_build.unwrap();
        let _res = self.crypto_keys_by_user_finder.run(user_id).await;
        todo!()
    }

    fn subscribet_to(&self) -> String {
        return FindCryptoKeysByUserQuery::QUERY_TYPE.to_string();
    }
}