pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: String) -> UserName {
        Self { value }
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Clone for UserName {
    fn clone(&self) -> Self {
        Self::new(self.value())
    }
}
