use std::vec;

use cqrs::domain::query_bus_response::QueryBusResponse;

use crate::{crypto_keys::domain::entities::crypto_key::CryptoKey, shared::domain::errors::DomainError};


pub struct CryptoKeyQueryResponse {
    pub data: Vec<CryptoKey>,
    pub errors: Vec<DomainError>,
}

impl CryptoKeyQueryResponse {
    pub const RES_TYPE: &'static str = "CryptoKeyQueryResponse";

    pub fn entities(data: Vec<CryptoKey>) -> CryptoKeyQueryResponse {
        CryptoKeyQueryResponse { data, errors: vec![] }
    }

    pub fn boxed_entities(data: Vec<CryptoKey>) -> Box<CryptoKeyQueryResponse> {
        let res = Self::entities(data);
        Box::new(res)
    }

    pub fn entity(data: CryptoKey) -> CryptoKeyQueryResponse {
        CryptoKeyQueryResponse { data: vec![data], errors: vec![] }
    }

    pub fn boxed_entity(data: CryptoKey) -> Box<CryptoKeyQueryResponse> {
        let res = Self::entity(data);
        Box::new(res)
    }

    pub fn errs(errors: Vec<DomainError>) -> CryptoKeyQueryResponse {
        CryptoKeyQueryResponse { data: vec![], errors }
    }

    pub fn boxed_errs(errors: Vec<DomainError>) -> Box<CryptoKeyQueryResponse> {
        let res = CryptoKeyQueryResponse::errs(errors);
        Box::new(res)
    }

    pub fn err(error: DomainError) -> CryptoKeyQueryResponse {
        CryptoKeyQueryResponse { data: vec![], errors: vec![error] }
    }

    pub fn boxed_err(error: DomainError) -> Box<CryptoKeyQueryResponse> {
        let res = CryptoKeyQueryResponse::err(error);
        Box::new(res)
    }
}

impl QueryBusResponse for CryptoKeyQueryResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }
}