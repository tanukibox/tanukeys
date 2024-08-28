use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum GeneralErrorTypes {
    ResourceNotFound,
    ResourceAlreadyExists,
    Other,
}

impl GeneralErrorTypes {
    fn to_string(&self) -> String {
        match self {
            GeneralErrorTypes::ResourceNotFound => "ResourceNotFound".to_string(),
            GeneralErrorTypes::ResourceAlreadyExists => "ResourceAlreadyExists".to_string(),
            GeneralErrorTypes::Other => "Other".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct DomainError {
    message: String,
    general_err_type: GeneralErrorTypes,
    specific_error_type: String,
}

impl DomainError {
    pub fn new(
        message: String,
        general_err_type: GeneralErrorTypes,
        specific_error_type: String,
    ) -> DomainError {
        Self {
            message,
            general_err_type,
            specific_error_type,
        }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn get_err_type(&self) -> &GeneralErrorTypes {
        &self.general_err_type
    }
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DomainError/{}/{}: {}",
            self.general_err_type.to_string(),
            self.specific_error_type,
            self.message
        )
    }
}

impl Error for DomainError {}
