use core::panic;

use crate::shared::domain::entities::user_id::UserId;
use crate::users::domain::{entities::user::User, errors::{user_already_exists_error::user_already_exists_error, user_not_found_error::user_not_found_error}, user_repository::UserRepository};
use async_trait::async_trait;
use sqlx::Error;

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
        sqlx::query("SET search_path TO kernel")
            .execute(&pool)
            .await.expect("Schema kernel not found.");
        Self::new(pool)
    }
}

#[async_trait]
impl UserRepository for SqlxPostgresUserRepository {
    async fn find_by_id(&self, id: &UserId) -> Result<User, Box<dyn std::error::Error>> {
        let query = sqlx::query_as("SELECT id, name FROM users WHERE id = $1")
            .bind(id.value());
        let user_res: Result<SqlxUser, Error> = query.fetch_one(&self.pool).await;
        if user_res.is_err() {
            return Err(Box::new(user_not_found_error(id.clone())));
        }
        Ok(user_res.unwrap().to_domain())
    }

    async fn create_one(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let sql_user: SqlxUser = SqlxUser::from_domain(user);
        let res = sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
            .bind(&sql_user.id)
            .bind(&sql_user.name)
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() { // TODO: check sql error code or message
            return Err(Box::new(user_already_exists_error(user.id.clone())));
        }
        Ok(())
    }

    async fn update_one(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        let sql_user: SqlxUser = SqlxUser::from_domain(user);
        let res = sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
            .bind(&sql_user.name)
            .bind(&sql_user.id)
            .fetch_optional(&self.pool)
            .await;

        if res.is_err() { // TODO: check sql error code or message
            return Err(Box::new(user_not_found_error(user.id.clone())));
        }

        Ok(())
    }

    async fn delete_one(&self, id: &UserId) -> Result<(), Box<dyn std::error::Error>> {
        let res = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() { // TODO: check sql error code or message
            return Err(Box::new(user_not_found_error(id.clone())));
        }
        Ok(())
    }
}