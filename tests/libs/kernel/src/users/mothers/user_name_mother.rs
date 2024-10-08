use std::iter;

use kernel::users::domain::entities::user_name::UserName;
use rand::Rng;



pub struct UserNameMother {}

impl UserNameMother {
    pub fn random() -> UserName {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();
        let gen_one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
        let name_size = rand::thread_rng().gen_range(5..15);
        let random_str: String = iter::repeat_with(gen_one_char).take(name_size).collect();
        UserName::new(random_str).unwrap()
    }

    pub fn with_params(value: Option<String>) -> UserName {
        match value {
            Some(value) => UserName::new(value).unwrap(),
            None => Self::random(),
        }
    }
}