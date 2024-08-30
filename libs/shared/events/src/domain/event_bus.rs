use async_trait::async_trait;

use super::{domain_event::DomainEvent, event_handler::EventHandler};

#[async_trait]
pub trait EventBus: Send + Sync + 'static {
    async fn publish(&self, event: Box<dyn DomainEvent>);
    async fn subscribe(&self, event_type: String, handler: Box<dyn EventHandler>);
}