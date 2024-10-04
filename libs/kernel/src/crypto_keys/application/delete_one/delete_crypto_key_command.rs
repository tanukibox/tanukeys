use cqrs::domain::command::Command;


pub struct DeleteCryptoKeyCommand {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub logged_user: Option<String>,
}

impl DeleteCryptoKeyCommand {
    pub const COMMAND_TYPE: &'static str = "DeleteCryptoKeyCommand";

    pub fn new(id: Option<String>, user_id: Option<String>, logged_user: Option<String>) -> DeleteCryptoKeyCommand {
        DeleteCryptoKeyCommand { id, user_id, logged_user }
    }
}

impl Command for DeleteCryptoKeyCommand {
    fn command_type(&self) -> String {
        DeleteCryptoKeyCommand::COMMAND_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}