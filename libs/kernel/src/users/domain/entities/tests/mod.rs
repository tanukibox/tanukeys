pub mod mothers;

#[cfg(test)]
mod create_simple_user {
    use crate::shared::domain::entities::user_id::UserId;

    use super::super::user::User;
    use super::super::user_name::UserName;
    use super::super::user_bio::UserBio;

    #[test]
    fn create_simple_user() {
        let id = UserId::new("".to_string()).unwrap();
        let name = UserName::new("tanukeys".to_string()).unwrap();
        let bio = UserBio::new(Some("I am a software engineer.".to_string())).unwrap();

        User::new(id, name, bio);
    }
}