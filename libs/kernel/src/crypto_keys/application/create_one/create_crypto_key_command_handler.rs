use std::sync::Arc;

use cqrs::domain::{command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler};
use events::domain::event_bus::EventBus;

use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;

use super::crypto_key_creator::CryptoKeyCreator;


pub struct CreateCryptoKeyCommandHandler<R: CryptoKeyRepository, E: EventBus> {
    _creator: Arc<CryptoKeyCreator<R, E>>,
}

impl<R: CryptoKeyRepository, E: EventBus>  CreateCryptoKeyCommandHandler<R, E> {
    pub fn new(creator: Arc<CryptoKeyCreator<R, E>>) -> CreateCryptoKeyCommandHandler<R, E> {
        CreateCryptoKeyCommandHandler { _creator: creator }
    }
}

impl <R: CryptoKeyRepository, E: EventBus> CommandHandler for CreateCryptoKeyCommandHandler<R, E> {
    fn handle(&self, _command: Box<dyn Command>) -> Box<dyn CommandBusResponse> {
        todo!()
    }
    
    fn subscribet_to(&self) -> String {
        todo!()
    }
}


