
pub trait DomainEvent {
    fn event_type(&self) -> String;
}