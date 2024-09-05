use std::error::Error;
use async_trait::async_trait;
use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::domain::errors::crypto_key_not_found_error::crypto_key_not_found_error;
use crate::crypto_keys::infrastructure::sqlx::sqlx_crypto_key::SqlxCryptoKey;
use crate::shared::domain::entities::user_id::UserId;

use tracing::{self as logger};

pub struct SqlxPostgresCryptoKeyRepository {
    pool: sqlx::PgPool,
}

impl SqlxPostgresCryptoKeyRepository {
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
impl CryptoKeyRepository for SqlxPostgresCryptoKeyRepository {
    async fn find_by_id(&self, user_id: &UserId, id: &CryptoKeyId) -> Result<CryptoKey, Box<dyn Error>> {
        let query = sqlx::query_as("SELECT id, name, payload, user_id FROM cryptokeys WHERE id = $1 AND user_id = $2")
            .bind(id.value())
            .bind(user_id.value());
        let user_res: Result<SqlxCryptoKey, sqlx::Error> = query.fetch_one(&self.pool).await;
        if user_res.is_err() {
            match user_res.err().unwrap() {
                sqlx::Error::Database(err) => {
                    let _msg = err.message();
                    let _code = err.code().unwrap();
                    logger::error!("{}: {}", "Database", err);
                }
                _ => {}
            }
            return Err(Box::new(crypto_key_not_found_error(user_id.clone(), id.clone())));
        }
        Ok(user_res.unwrap().to_domain())
    }

    async fn create_one(&self, key: &CryptoKey) -> Result<(), Box<dyn Error>> {
        let sql_user: SqlxCryptoKey = SqlxCryptoKey::from_domain(key);
        let res = sqlx::query("INSERT INTO cryptokeys (id, name, payload, user_id) VALUES ($1, $2, $3, $4)")
            .bind(&sql_user.id)
            .bind(&sql_user.name)
            .bind(&sql_user.payload)
            .bind(&sql_user.user_id)
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() { // TODO: check sql error code or message
            match res.err().unwrap() {
                sqlx::Error::Database(err) => {
                    let _msg = err.message();
                    let _code = err.code().unwrap();
                    logger::error!("{}: {}", "Database", err);
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn update_one(&self, _user: &CryptoKey) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn delete_one(&self, _user_id: &UserId, _id: &CryptoKeyId) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

