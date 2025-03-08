#[cfg(test)]
mod user_mother_stress_tests {
    use crate::users::mothers::user_mother::UserMother;

    const ITERATIONS: usize = 100;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let user = UserMother::random();
            assert!(!user.id.value().is_empty());
            assert!(!user.name.value().is_empty());
        }
    }
} 