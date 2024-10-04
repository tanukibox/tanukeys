use std::any::Any;

use cqrs::domain::query::Query;


pub struct FindCryptoKeyQuery {
    pub user_id: Option<String>,
    pub crypto_key_id: Option<String>,
}

impl FindCryptoKeyQuery {
    pub const QUERY_TYPE: &'static str = "FindCryptoKeyQuery";

    pub fn new(user_id: Option<String>, crypto_key_id: Option<String>) -> FindCryptoKeyQuery {
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