use std::sync::Arc;

use async_trait::async_trait;
use cqrs::domain::{command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler};
use events::domain::event_bus::EventBus;

use crate::{crypto_keys::{application::crypto_key_command_response::CryptoKeyCommandResponse, domain::{crypto_key_repository::CryptoKeyRepository, entities::{crypto_key_description::CryptoKeyDescription, crypto_key_id::CryptoKeyId, crypto_key_name::CryptoKeyName, crypto_key_payload::CryptoKeyPayload}}}, shared::domain::entities::user_id::UserId};

use super::{create_crypto_key_command::CreateCryptoKeyCommand, crypto_key_creator::CryptoKeyCreator};


pub struct CreateCryptoKeyCommandHandler<R: CryptoKeyRepository, E: EventBus> {
    creator: Arc<CryptoKeyCreator<R, E>>,
}

impl<R: CryptoKeyRepository, E: EventBus>  CreateCryptoKeyCommandHandler<R, E> {
    pub fn new(creator: Arc<CryptoKeyCreator<R, E>>) -> CreateCryptoKeyCommandHandler<R, E> {
        CreateCryptoKeyCommandHandler { creator }
    }
}

#[async_trait]
impl <R: CryptoKeyRepository, E: EventBus> CommandHandler for CreateCryptoKeyCommandHandler<R, E> {
    async fn handle(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse> {
        let command = command.as_any().downcast_ref::<CreateCryptoKeyCommand>().unwrap();

        let id = CryptoKeyId::new(command.id.clone());
        let name = CryptoKeyName::new(command.name.clone());
        let payload = CryptoKeyPayload::new(command.payload.clone());
        let user_id = UserId::new(command.user_id.clone());
        let description = CryptoKeyDescription::new(command.description.clone());
        let logged_user = UserId::new(command.logged_user.clone());

        if id.is_err() {
            return CryptoKeyCommandResponse::boxed_err(id.err().unwrap());
        }
        if name.is_err() {
            return CryptoKeyCommandResponse::boxed_err(name.err().unwrap());
        }
        if payload.is_err() {
            return CryptoKeyCommandResponse::boxed_err(payload.err().unwrap());
        }
        if user_id.is_err() {
            return CryptoKeyCommandResponse::boxed_err(user_id.err().unwrap());
        }
        if description.is_err() {
            return CryptoKeyCommandResponse::boxed_err(description.err().unwrap());
        }
        if logged_user.is_err() {
            return CryptoKeyCommandResponse::boxed_err(logged_user.err().unwrap());
        }

        let id = id.unwrap();
        let name = name.unwrap();
        let payload = payload.unwrap();
        let user_id = user_id.unwrap();
        let description = description.unwrap();
        let logged_user = logged_user.unwrap();

        self.creator.run(id, name, payload, user_id, description, logged_user).await;

        CryptoKeyCommandResponse::boxed_ok()
    }
    
    fn subscribet_to(&self) -> String {
        return CreateCryptoKeyCommand::COMMAND_TYPE.to_string();
    }
}


