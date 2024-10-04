
use cqrs::domain::command_bus_response::CommandBusResponse;

use crate::shared::domain::errors::DomainError;


pub struct CryptoKeyCommandResponse {
    pub error: Option<DomainError>,
}

impl CryptoKeyCommandResponse {
    pub const RES_TYPE: &'static str = "CryptoKeyCommandResponse";

    pub fn ok() -> CryptoKeyCommandResponse {
        CryptoKeyCommandResponse { error: None }
    }

    pub fn boxed_ok() -> Box<CryptoKeyCommandResponse> {
        let res = CryptoKeyCommandResponse::ok();
        Box::new(res)
    }

    pub fn err(error: DomainError) -> CryptoKeyCommandResponse {
        CryptoKeyCommandResponse { error: Some(error) }
    }

    pub fn boxed_err(error: DomainError) -> Box<CryptoKeyCommandResponse> {
        let res = CryptoKeyCommandResponse::err(error);
        Box::new(res)
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }
}

impl CommandBusResponse for CryptoKeyCommandResponse {
    fn response_type(&self) -> String {
        Self::RES_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}