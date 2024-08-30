use std::collections::HashMap;

use crate::domain::{domain_event::DomainEvent, event_bus::EventBus, event_handler::EventHandler};


pub struct InMemoryEventBus {
    pub handlers_by_sub: HashMap<String, Vec<Box<dyn EventHandler>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        InMemoryEventBus {
            handlers_by_sub: HashMap::new(),
        }
    }
}

impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: Box<dyn DomainEvent>) {
        let event_type = event.event_type();
        if let Some(handlers) = self.handlers_by_sub.get(&event_type) {
            for handler in handlers {
                handler.handle(event.clone());
            }
        }
    }

    fn subscribe(&mut self, event_type: String, handler: Box<dyn EventHandler>) {
        let handlers = self.handlers_by_sub.entry(event_type).or_insert(vec![]);
        handlers.push(handler);
    }
}