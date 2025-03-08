#[cfg(test)]
mod user_bio_mother_stress_tests {
    use crate::users::mothers::user_bio_mother::UserBioMother;

    const ITERATIONS: usize = 1000;

    #[test]
    fn test_random_generation() {
        for _ in 0..ITERATIONS {
            let bio = UserBioMother::random();
            let _ = bio.value();
        }
    }
} 