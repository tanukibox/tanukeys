use std::error::Error;

pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: String) -> Result<UserName, Box<dyn Error>> {
        Ok(Self { value })
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for UserName {
    fn clone(&self) -> Self {
        let res = Self::new(self.value());
        res.unwrap()
    }
}
