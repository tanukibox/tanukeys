use super::{command::Command, command_bus_response::CommandBusResponse};



pub trait CommandBus {
    fn dispatch(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse>;
}