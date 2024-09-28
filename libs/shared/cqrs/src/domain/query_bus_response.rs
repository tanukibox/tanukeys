
pub trait QueryBusResponse: Send + Sync {
    fn response_type(&self) -> String;
}