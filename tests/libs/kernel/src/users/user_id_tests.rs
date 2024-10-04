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
        let res = UserId::new(Some("".to_string()));
        assert!(res.is_err())
    }

    #[test]
    fn create_with_special_chars() {
        let res = UserId::new(Some("tanuki_box".to_string()));
        assert!(res.is_ok())
    }

    #[test]
    fn create_with_one_upper_case_id() {
        let res = UserId::new(Some("tanukiBox".to_string()));
        assert!(res.is_err())
    }

    #[test]
    fn create_with_two_upper_case_id() {
        let res = UserId::new(Some("TanukiBox".to_string()));
        assert!(res.is_err())
    }
    
    #[test]
    fn create_all_upper_case_id() {
        let res = UserId::new(Some("TANUKIBOX".to_string()));
        assert!(res.is_err())
    }

    #[test]
    fn create_with_special_char() {
        let res = UserId::new(Some("tanuki_box".to_string()));
        assert!(res.is_ok())
    }

    #[test]
    fn create_with_just_not_allowed_char() {
        let res = UserId::new(Some(";".to_string()));
        assert!(res.is_err())
    }

    #[test]
    fn create_with_not_allowed_char() {
        let res = UserId::new(Some("tanuki;box".to_string()));
        assert!(res.is_err())
    }

    #[test]
    fn create_just_with_space_char() {
        let res = UserId::new(Some(" ".to_string()));
        assert!(res.is_err())
    }

    #[test]
    fn create_with_space_char() {
        let res = UserId::new(Some("tanuki box".to_string()));
        assert!(res.is_err())
    }

    #[test]
    fn create_with_some_not_allowed_char() {
        let res = UserId::new(Some("tanuki box!".to_string()));
        assert!(res.is_err())
    }

}