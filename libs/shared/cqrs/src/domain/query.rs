
pub trait Query: Send + Sync {
    fn get_type(&self) -> String;
}