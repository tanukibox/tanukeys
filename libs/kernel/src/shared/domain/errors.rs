use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    //                    GENERAL ERRORS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[error("Unknown error.")]
    Unknown,

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    //                     USER ERRORS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[error("User with id <{user_id:?}> already exists.")]
    UserAlreadyExists { user_id: String },

    #[error("User with id <{user_id:?}> not found.")]
    UserNotFound { user_id: String },

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    //                  CRYPTO KEY ERRORS
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    #[error("CryptoKey with id <{id:?}> and user id <{user_id:?}> already exists.")]
    CryptoKeyAlreadyExists { id: String, user_id: String },

    #[error("CryptoKey with id <{id:?}> and user id <{user_id:?}> not found.")]
    CryptoKeyNotFound { id: String, user_id: String },
    
}