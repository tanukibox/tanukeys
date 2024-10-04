use std::sync::Arc;

use async_trait::async_trait;
use cqrs::domain::{command::Command, command_bus_response::CommandBusResponse, command_handler::CommandHandler};
use events::domain::event_bus::EventBus;

use crate::{crypto_keys::{application::crypto_key_command_response::CryptoKeyCommandResponse, domain::{crypto_key_repository::CryptoKeyRepository, entities::crypto_key_id::CryptoKeyId}}, shared::domain::entities::user_id::UserId};

use super::{crypto_key_deleter::CryptoKeyDeleter, delete_crypto_key_command::DeleteCryptoKeyCommand};

pub struct CreateCryptoKeyCommandHandler<R: CryptoKeyRepository, E: EventBus> {
    deleter: Arc<CryptoKeyDeleter<R, E>>,
}

impl<R: CryptoKeyRepository, E: EventBus>  CreateCryptoKeyCommandHandler<R, E> {
    pub fn new(deleter: Arc<CryptoKeyDeleter<R, E>>) -> CreateCryptoKeyCommandHandler<R, E> {
        CreateCryptoKeyCommandHandler { deleter }
    }
}

#[async_trait]
impl <R: CryptoKeyRepository, E: EventBus> CommandHandler for CreateCryptoKeyCommandHandler<R, E> {
    async fn handle(&self, command: Box<dyn Command>) -> Box<dyn CommandBusResponse> {
        let command = command.as_any().downcast_ref::<DeleteCryptoKeyCommand>().unwrap();

        let id = CryptoKeyId::new(command.id.clone());
        let user_id = UserId::new(command.user_id.clone());
        let logged_user = UserId::new(command.logged_user.clone());

        if id.is_err() {
            return CryptoKeyCommandResponse::boxed_err(id.err().unwrap());
        }
        if user_id.is_err() {
            return CryptoKeyCommandResponse::boxed_err(user_id.err().unwrap());
        }
        if logged_user.is_err() {
            return CryptoKeyCommandResponse::boxed_err(logged_user.err().unwrap());
        }

        let id = id.unwrap();
        let user_id = user_id.unwrap();
        let logged_user = logged_user.unwrap();

        let res = self.deleter.run(id, user_id, logged_user).await;

        match res {
            Ok(_) => CryptoKeyCommandResponse::boxed_ok(),
            Err(err) => CryptoKeyCommandResponse::boxed_err(err)
        }
    }
    
    fn subscribet_to(&self) -> String {
        return DeleteCryptoKeyCommand::COMMAND_TYPE.to_string();
    }
}


