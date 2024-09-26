use super::{query::Query, query_bus_response::QueryBusResponse};


pub trait QueryHandler<Q, R> {
    fn handle(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse>;
}