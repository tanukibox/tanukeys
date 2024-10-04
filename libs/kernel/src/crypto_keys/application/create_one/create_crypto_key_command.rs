use cqrs::domain::command::Command;


pub struct CreateCryptoKeyCommand {
    pub id: Option<String>,
    pub name: Option<String>,
    pub payload: Option<String>,
    pub user_id: Option<String>,
    pub description: Option<String>,
    pub logged_user: Option<String>,
}

impl CreateCryptoKeyCommand {
    pub const COMMAND_TYPE: &'static str = "CreateCryptoKeyCommand";

    pub fn new(id: Option<String>, name: Option<String>, payload: Option<String>, user_id: Option<String>, description: Option<String>, 
        logged_user: Option<String>) -> CreateCryptoKeyCommand {
            
        CreateCryptoKeyCommand { id, name, payload, user_id, description, logged_user }
    }
}

impl Command for CreateCryptoKeyCommand {
    fn command_type(&self) -> String {
        CreateCryptoKeyCommand::COMMAND_TYPE.to_string()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}