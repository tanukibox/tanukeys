use crate::shared::domain::errors::DomainError;

pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: Option<String>) -> Result<UserName, DomainError> {
        if value.is_none() {
            return Err(DomainError::ValueObjectError { value: "User name must not be null".to_string() })
        }
        let value = value.unwrap();
        Ok(Self { value })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for UserName {
    fn clone(&self) -> Self {
        let res = Self::new(Some(self.value()));
        res.unwrap()
    }
}
