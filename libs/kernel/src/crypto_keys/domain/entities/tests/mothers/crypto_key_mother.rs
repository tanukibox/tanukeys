use crate::{crypto_keys::domain::entities::{crypto_key::CryptoKey, crypto_key_description::CryptoKeyDescription}, users::domain::entities::tests::mothers::user_id_mother::UserIdMother};

use super::{crypto_key_description_mother::CryptoKeyDescriptionMother, crypto_key_id_mother::CryptoKeyIdMother, crypto_key_name_mother::CryptoKeyNameMother, crypto_key_payload_mother::CryptoKeyPayloadMother};


pub struct CryptoKeyMother {}

impl CryptoKeyMother {
    pub fn random() -> CryptoKey {
        let id = CryptoKeyIdMother::random();
        let name = CryptoKeyNameMother::random();
        let description = CryptoKeyDescriptionMother::random();
        let payload = CryptoKeyPayloadMother::random();
        let user_id = UserIdMother::random();

        CryptoKey::new(id, name, payload, user_id, description)
    }

    pub fn with_params(
        id: Option<String>,
        name: Option<String>,
        description: Option<String>,
        payload: Option<String>,
        user_id: Option<String>,
    ) -> CryptoKey {
        let id = CryptoKeyIdMother::with_params(id);
        let name = CryptoKeyNameMother::with_params(name);
        let description: CryptoKeyDescription = CryptoKeyDescriptionMother::with_params(description);
        let payload = CryptoKeyPayloadMother::with_params(payload);
        let user_id = UserIdMother::with_params(user_id);

        CryptoKey::new(id, name, payload, user_id, description)
    }
}