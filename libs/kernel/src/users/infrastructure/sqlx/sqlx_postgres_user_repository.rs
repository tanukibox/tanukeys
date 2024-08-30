use core::panic;

use sqlx::Error;

use crate::users::domain::{errors::user_not_found_error::user_not_found_error, user_repository::UserRepository};

use super::sqlx_user::SqlxUser;


pub struct SqlxPostgresUserRepository {
    pool: sqlx::PgPool,
}

impl SqlxPostgresUserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn from_env() -> Self {
        let url_load_res = std::env::var("DATABASE_URL");
        if url_load_res.is_err() {
            panic!("DATABASE_URL not found in environment variables.");
        }
        let url = url_load_res.unwrap();
        let pool_res = sqlx::PgPool::connect(&url).await;
        if pool_res.is_err() {
            panic!("Failed to connect to database: {:?}", pool_res.err());
        }
        let pool = pool_res.unwrap();
        Self::new(pool)
    }
}

impl UserRepository for SqlxPostgresUserRepository {
    async fn find_by_id(&self, id: &crate::users::domain::entities::user_id::UserId) -> Result<crate::users::domain::entities::user::User, Box<dyn std::error::Error>> {
        let query = sqlx::query_as("SELECT id, name FROM users WHERE id = $1").bind(id.value());
        let user_res: Result<SqlxUser, Error> = query.fetch_one(&self.pool).await;
        if user_res.is_err() {
            return Err(Box::new(user_not_found_error(id.clone())));
        }
        Ok(user_res.unwrap().to_domain())
    }

    async fn create_one(&self, _user: &crate::users::domain::entities::user::User) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn update_one(&self, _user: &crate::users::domain::entities::user::User) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn delete_one(&self, _id: &crate::users::domain::entities::user_id::UserId) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}