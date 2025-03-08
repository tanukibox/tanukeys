#[cfg(test)]
mod user_name_mother_stress_tests {
    use crate::users::mothers::user_name_mother::UserNameMother;

    const ITERATIONS: usize = 100;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let name = UserNameMother::random();
            let value = name.value();
            assert!(!value.is_empty());
        }
    }
} 