#[derive(Debug)]
pub enum CryptoKeyErrorTypes {
    CryptoKeyAlreadyExists,
    CryptoKeyNotFound,
}

impl CryptoKeyErrorTypes {
    pub fn to_string(&self) -> String {
        match self {
            CryptoKeyErrorTypes::CryptoKeyAlreadyExists => "CryptoKeyAlreadyExists".to_string(),
            CryptoKeyErrorTypes::CryptoKeyNotFound => "CryptoKeyNotFound".to_string(),
        }
    }
}
