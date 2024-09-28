use cqrs::domain::query::Query;


pub struct FindCryptoKeysByUserQuery {
    pub user_id: String,
}

impl FindCryptoKeysByUserQuery {
    pub const QUERY_TYPE: &'static str = "FindCryptoKeysByUserQuery";

    pub fn new(user_id: String) -> FindCryptoKeysByUserQuery {
        FindCryptoKeysByUserQuery { user_id }
    }
}

impl Query for FindCryptoKeysByUserQuery {
    fn get_type(&self) -> String {
        FindCryptoKeysByUserQuery::QUERY_TYPE.to_string()
    }
}