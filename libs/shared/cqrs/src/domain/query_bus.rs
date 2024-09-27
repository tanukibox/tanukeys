use super::{query::Query, query_bus_response::QueryBusResponse};



pub trait QueryBus {
    fn ask(&self, query: Box<dyn Query>) -> Box<dyn QueryBusResponse>;
}