use crate::crypto_keys::domain::entities::{crypto_key::CryptoKey, crypto_key_description::CryptoKeyDescription};
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::entities::crypto_key_name::CryptoKeyName;
use crate::crypto_keys::domain::entities::crypto_key_payload::CryptoKeyPayload;
use crate::shared::domain::entities::user_id;
use crate::shared::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CryptoKeyJsonDto {
    pub id: String,
    pub name: String,
    pub payload: String,
    pub user_id: String,
    pub description: String,
}

pub fn parse_to_dto(key: &CryptoKey) -> CryptoKeyJsonDto {
    CryptoKeyJsonDto {
        id: key.id.value(),
        name: key.name.value(),
        payload: key.payload.value(),
        user_id: key.user_id.value(),
        description: key.description.value(),
    }
}

pub fn parse_to_domain(dto: &CryptoKeyJsonDto) -> Result<CryptoKey, DomainError> {
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
    let key_description = CryptoKeyDescription::new(dto.description.clone());
    if key_description.is_err() {
        return Err(key_description.err().unwrap())
    }
    Ok(CryptoKey::new(id?, name?, payload?, user_id?, key_description?))
}
