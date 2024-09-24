
use kernel::users::domain::entities::user::User;

use super::{user_bio_mother::UserBioMother, user_id_mother::UserIdMother, user_name_mother::UserNameMother};


pub struct UserMother {}

impl UserMother {
    pub fn random() -> User {
        let id = UserIdMother::random();
        let name = UserNameMother::random();
        let bio = UserBioMother::random();

        User::new(id, name, bio)
    }

    pub fn with_params(
        id: Option<String>,
        name: Option<String>,
        bio: Option<Option<String>>,
    ) -> User {
        let id = UserIdMother::with_params(id);
        let name = UserNameMother::with_params(name);
        let bio = UserBioMother::with_params(bio);

        User::new(id, name, bio)
    }
}