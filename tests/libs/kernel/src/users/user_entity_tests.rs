#[cfg(test)]
pub mod user_entity {

    use crate::users::mothers::user_mother::UserMother;


    #[test]
    fn create_simple_user() {
        UserMother::random();
    }

}