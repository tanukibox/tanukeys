use core::panic;

use super::sqlx_user::SqlxUser;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use crate::users::domain::{entities::user::User, user_repository::UserRepository};
use async_trait::async_trait;
use sqlx::Error;

use tracing::{self as logger, error};

#[derive(Debug)]
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
        logger::info!("Database connection success.");
        Self::new(pool)
    }
}

#[async_trait]
impl UserRepository for SqlxPostgresUserRepository {

    async fn find_by_id(&self, id: &UserId) -> Result<User, DomainError> {
        let query = sqlx::query_as("SELECT id, name FROM kernel.users WHERE id = $1")
            .bind(id.value());
        let user_res: Result<SqlxUser, Error> = query.fetch_one(&self.pool).await;
        if user_res.is_err() {
            return match user_res.err().unwrap() {
                Error::RowNotFound => Err(DomainError::UserNotFound { user_id: id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(user_res.unwrap().to_domain())
    }

    async fn create_one(&self, user: &User) -> Result<(), DomainError> {
        let sql_user: SqlxUser = SqlxUser::from_domain(user);
        let res = sqlx::query("INSERT INTO kernel.users (id, name) VALUES ($1, $2)")
            .bind(&sql_user.id)
            .bind(&sql_user.name)
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() {
            return match res.err().unwrap() {
                Error::Database(_) => Err(DomainError::UserAlreadyExists { user_id: user.id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(())
    }

    async fn update_one(&self, user: &User) -> Result<(), DomainError> {
        let sql_user: SqlxUser = SqlxUser::from_domain(user);
        let res = sqlx::query("UPDATE kernel.users SET name = $1 WHERE id = $2")
            .bind(&sql_user.name)
            .bind(&sql_user.id)
            .fetch_optional(&self.pool)
            .await;

        if res.is_err() { // TODO: check sql error code or message
            return match res.err().unwrap() {
                Error::RowNotFound => Err(DomainError::UserNotFound { user_id: user.id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }

        Ok(())
    }

    async fn delete_one(&self, id: &UserId) -> Result<(), DomainError> {
        let res = sqlx::query("DELETE FROM kernel.users WHERE id = $1")
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() { // TODO: check sql error code or message
            return match res.err().unwrap() {
                Error::RowNotFound => Err(DomainError::UserNotFound { user_id: id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(())
    }
}