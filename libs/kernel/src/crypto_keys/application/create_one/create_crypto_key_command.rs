use cqrs::domain::command::Command;


pub struct CreateCryptoKeyCommand {
    pub id: String,
    pub name: String,
    pub payload: String,
    pub user_id: String,
    pub description: String,
    pub logged_user: String,
}

impl CreateCryptoKeyCommand {
    pub fn new(id: String, name: String, payload: String, user_id: String, description: String, logged_user: String) -> CreateCryptoKeyCommand {
        CreateCryptoKeyCommand { id, name, payload, user_id, description, logged_user }
    }
}

impl Command for CreateCryptoKeyCommand {
    fn command_type(&self) -> String {
        "CreateCryptoKeyCommand".to_string()
    }
}