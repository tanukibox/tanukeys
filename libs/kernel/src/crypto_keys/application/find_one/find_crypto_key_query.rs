use std::any::Any;

use cqrs::domain::query::Query;


pub struct FindCryptoKeyQuery {
    pub user_id: String,
    pub crypto_key_id: String,
}

impl FindCryptoKeyQuery {
    pub const QUERY_TYPE: &'static str = "FindCryptoKeyQuery";

    pub fn new(user_id: String, crypto_key_id: String) -> FindCryptoKeyQuery {
        FindCryptoKeyQuery { user_id, crypto_key_id }
    }
}

impl Query for FindCryptoKeyQuery {
    fn get_type(&self) -> String {
        FindCryptoKeyQuery::QUERY_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}