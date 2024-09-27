use cqrs::domain::query::Query;


pub struct FindCryptoKeysByUserQuery {
    pub user_id: String,
}

impl FindCryptoKeysByUserQuery {
    pub fn new(user_id: String) -> FindCryptoKeysByUserQuery {
        FindCryptoKeysByUserQuery { user_id }
    }
}

impl Query for FindCryptoKeysByUserQuery {
    fn get_type(&self) -> String {
        "FindCryptoKeysByUserQuery".to_string()
    }
}