use std::iter;

use rand::Rng;

use crate::users::domain::entities::user_bio::UserBio;


pub struct UserBioModer {}

impl UserBioModer {
    pub fn random() -> UserBio {
        let name_size = rand::thread_rng().gen_range(0..150);
        if name_size == 0 {
            return UserBio::new(None).unwrap();
        }

        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ ";
        let mut rng = rand::thread_rng();
        let gen_one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;

        let random_str: String = iter::repeat_with(gen_one_char).take(name_size).collect();
        let random_str = Some(random_str);
        UserBio::new(random_str).unwrap()
    }

    pub fn create(value: Option<String>) -> UserBio {
        UserBio::new(value).unwrap()
    }

    pub fn with_params(value: Option<String>) -> UserBio {
        match value {
            Some(_) => UserBio::new(value).unwrap(),
            None => Self::random(),
        }
    }
}