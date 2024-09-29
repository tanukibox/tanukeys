use std::{collections::HashMap, hash::RandomState};

use actix_web::HttpResponse;
use kernel::shared::domain::errors::DomainError;


pub fn parse_domain_error(error: &DomainError) -> HttpResponse {
    match error {
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        //                    GENERAL ERRORS
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        DomainError::Unknown => {
            HttpResponse::InternalServerError().json(HashMap::<&str, &str, RandomState>::from_iter(vec![
                ("error", "Unknown"),
            ]))
        },
        DomainError::ValueObjectError { value } => {
            HttpResponse::BadRequest().json(HashMap::<&str, &str, RandomState>::from_iter(vec![
                ("error", "ValueObjectError"),
                ("value", &value),
            ]))
        },

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        //                     USER ERRORS
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        DomainError::UserAlreadyExists { user_id } => {
            HttpResponse::Conflict().json(HashMap::<&str, &str, RandomState>::from_iter(vec![
                ("error", "UserAlreadyExists"),
                ("user_id", &user_id),
            ]))
        },
        DomainError::UserNotAuthorized { user_id } => {
            HttpResponse::Unauthorized().json(HashMap::<&str, &str, RandomState>::from_iter(vec![
                ("error", "UserNotAuthorized"),
                ("user_id", &user_id),
            ]))
        },
        DomainError::UserNotFound { user_id } => {
            HttpResponse::NotFound().json(HashMap::<&str, &str, RandomState>::from_iter(vec![
                ("error", "UserNotFound"),
                ("user_id", &user_id),
            ]))
        },

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - -
        //                  CRYPTO KEY ERRORS
        // - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        DomainError::CryptoKeyAlreadyExists { id, user_id } => {
            HttpResponse::Conflict().json(HashMap::<&str, &str, RandomState>::from_iter(vec![
                ("error", "CryptoKeyAlreadyExists"),
                ("id", &id),
                ("user_id", &user_id),
            ]))
        },
        DomainError::CryptoKeyNotFound { id, user_id } => {
            HttpResponse::NotFound().json(HashMap::<&str, &str, RandomState>::from_iter(vec![
                ("error", "CryptoKeyNotFound"),
                ("id", &id),
                ("user_id", &user_id),
            ]))
        },
    }
}