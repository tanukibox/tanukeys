use std::vec;

use cqrs::domain::query_bus_response::QueryBusResponse;

use crate::{crypto_keys::domain::entities::crypto_key::CryptoKey, shared::domain::errors::DomainError};


pub struct CryptoKeyQueryResponse {
    pub data: Vec<CryptoKey>,
    pub error: Option<DomainError>,
}

impl CryptoKeyQueryResponse {
    pub const RES_TYPE: &'static str = "CryptoKeyQueryResponse";

    pub fn entities(data: Vec<CryptoKey>) -> CryptoKeyQueryResponse {
        CryptoKeyQueryResponse { data, error: None }
    }

    pub fn boxed_entities(data: Vec<CryptoKey>) -> Box<CryptoKeyQueryResponse> {
        let res = Self::entities(data);
        Box::new(res)
    }

    pub fn entity(data: CryptoKey) -> CryptoKeyQueryResponse {
        CryptoKeyQueryResponse { data: vec![data], error: None }
    }

    pub fn boxed_entity(data: CryptoKey) -> Box<CryptoKeyQueryResponse> {
        let res = Self::entity(data);
        Box::new(res)
    }

    pub fn err(error: DomainError) -> CryptoKeyQueryResponse {
        CryptoKeyQueryResponse { data: vec![], error: Some(error) }
    }

    pub fn boxed_err(error: DomainError) -> Box<CryptoKeyQueryResponse> {
        let res = CryptoKeyQueryResponse::err(error);
        Box::new(res)
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }
}

impl QueryBusResponse for CryptoKeyQueryResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}