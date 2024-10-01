
pub trait CommandBusResponse: Send + Sync {
    fn response_type(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}