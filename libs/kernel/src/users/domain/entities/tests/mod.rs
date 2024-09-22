pub mod mothers;

#[cfg(test)]
mod create_simple_user {

    use super::mothers::user_mother::UserMother;

    #[test]
    fn create_simple_user() {
        UserMother::random();
    }
}