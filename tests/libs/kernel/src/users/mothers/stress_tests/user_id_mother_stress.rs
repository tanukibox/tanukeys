#[cfg(test)]
mod user_id_mother_stress_tests {
    use crate::users::mothers::user_id_mother::UserIdMother;

    const ITERATIONS: usize = 100;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let id = UserIdMother::random();
            assert!(!id.value().is_empty());
        }
    }
} 