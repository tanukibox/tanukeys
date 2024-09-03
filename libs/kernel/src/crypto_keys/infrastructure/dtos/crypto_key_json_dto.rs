use crate::shared::domain::entities::user_id;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;

#[derive(Clone, Serialize, Deserialize)]
pub struct CryptoKeyJsonDto {
    pub id: String,
    pub name: String,
    pub payload: String,
    pub user_id: String,
}

pub fn parse_to_dto(key: &CryptoKey) -> CryptoKeyJsonDto {
    CryptoKeyJsonDto {
        id: key.id.value(),
        name: key.name.value(),
        payload: key.payload.value(),
        user_id: key.user_id.value(),
    }
}

pub fn parse_to_domain(dto: &CryptoKeyJsonDto) -> Result<CryptoKey, Box<dyn Error>> {
    let id = CryptoKeyId::new(dto.id.clone());
    if id.is_err() {
        return Err(id.err().unwrap())
    }
    let name = CryptoKeyName::new(dto.name.clone());
    if name.is_err() {
        return Err(name.err().unwrap())
    }
    let payload = CryptoKeyPayload::new(dto.payload.clone());
    if payload.is_err() {
        return Err(payload.err().unwrap())
    }
    let user_id = user_id::UserId::new(dto.user_id.clone());
    if user_id.is_err() {
        return Err(user_id.err().unwrap())
    }
    Ok(CryptoKey::new(id?, name?, payload?, user_id?))
}
