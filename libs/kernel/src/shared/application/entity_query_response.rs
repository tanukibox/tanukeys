use std::vec;

use cqrs::domain::query_bus_response::QueryBusResponse;

use crate::shared::domain::errors::DomainError;


pub struct EntityQueryResponse<E> {
    pub data: Vec<E>,
    pub errors: Vec<DomainError>,
}

impl<E> EntityQueryResponse<E> {
    pub const RES_TYPE: &'static str = "EntityQueryResponse";

    pub fn entities(data: Vec<E>) -> EntityQueryResponse<E> {
        EntityQueryResponse { data, errors: vec![] }
    }

    pub fn boxed_entities(data: Vec<E>) -> Box<EntityQueryResponse<E>> {
        let res = Self::entities(data);
        Box::new(res)
    }

    pub fn entity(data: E) -> EntityQueryResponse<E> {
        EntityQueryResponse { data: vec![data], errors: vec![] }
    }

    pub fn boxed_entity(data: E) -> Box<EntityQueryResponse<E>> {
        let res = Self::entity(data);
        Box::new(res)
    }

    pub fn errs(errors: Vec<DomainError>) -> EntityQueryResponse<E> {
        EntityQueryResponse { data: vec![], errors }
    }

    pub fn boxed_errs(errors: Vec<DomainError>) -> Box<EntityQueryResponse<E>> {
        let res = EntityQueryResponse::errs(errors);
        Box::new(res)
    }

    pub fn err(error: DomainError) -> EntityQueryResponse<E> {
        EntityQueryResponse { data: vec![], errors: vec![error] }
    }

    pub fn boxed_err(error: DomainError) -> Box<EntityQueryResponse<E>> {
        let res = EntityQueryResponse::err(error);
        Box::new(res)
    }
}

impl<E> QueryBusResponse for EntityQueryResponse<E>
where E: Send + Sync {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }
}