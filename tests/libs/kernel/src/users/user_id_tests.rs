#[cfg(test)]
pub mod user_id {
    use kernel::shared::domain::entities::user_id::UserId;

    use crate::users::mothers::user_id_mother::UserIdMother;




    #[test]
    fn create_random_valid_id() {
        UserIdMother::random();
    }

    #[test]
    fn create_empty_id() {
        let res = UserId::new("".to_string());
        assert!(res.is_err())
    }

}