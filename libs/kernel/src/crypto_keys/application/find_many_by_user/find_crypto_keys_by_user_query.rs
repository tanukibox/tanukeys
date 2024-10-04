use std::any::Any;

use cqrs::domain::query::Query;


pub struct FindCryptoKeysByUserQuery {
    pub user_id: Option<String>,
}

impl FindCryptoKeysByUserQuery {
    pub const QUERY_TYPE: &'static str = "FindCryptoKeysByUserQuery";

    pub fn new(user_id: Option<String>) -> FindCryptoKeysByUserQuery {
        FindCryptoKeysByUserQuery { user_id }
    }
}

impl Query for FindCryptoKeysByUserQuery {
    fn get_type(&self) -> String {
        FindCryptoKeysByUserQuery::QUERY_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}