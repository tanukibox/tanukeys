#[cfg(test)]
pub mod user_bio {
    use kernel::users::domain::entities::user_bio::UserBio;

    use crate::users::mothers::user_bio_mother::UserBioMother;


    #[test]
    fn create_random_valid_bio() {
        UserBioMother::random();
    }

    #[test]
    fn create_empty_bio() {
        let res = UserBio::new(Some("".to_string()));
        assert!(res.is_ok())
    }

    #[test]
    fn create_null_bio() {
        let res = UserBio::new(None);
        assert!(res.is_ok())
    }

    #[test]
    fn create_japanese_bio() {
        let bio_str = Some("こんにちは".to_string());
        let res = UserBio::new(bio_str);
        assert!(res.is_ok())
    }

    #[test]
    fn create_corean_bio() {
        let bio_str = Some("안녕하세요".to_string());
        let res = UserBio::new(bio_str);
        assert!(res.is_ok())
    }

    #[test]
    fn create_chinese_bio() {
        let bio_str = Some("你好".to_string());
        let res = UserBio::new(bio_str);
        assert!(res.is_ok())
    }

    #[test]
    fn create_arabic_bio() {
        let bio_str = Some("مرحبا".to_string());
        let res = UserBio::new(bio_str);
        assert!(res.is_ok())
    }

}