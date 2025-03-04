use async_trait::async_trait;
use crate::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use crate::crypto_keys::domain::entities::crypto_key::CryptoKey;
use crate::crypto_keys::domain::entities::crypto_key_id::CryptoKeyId;
use crate::crypto_keys::infrastructure::sqlx::sqlx_crypto_key::SqlxCryptoKey;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;

use tracing::error;

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

    async fn find_many(&self, user_id: &UserId) -> Result<Vec<CryptoKey>, DomainError> {
        let query = sqlx::query_as("SELECT id, name, payload, user_id, description, key_type, domain, status FROM cryptokeys WHERE user_id = $1")
            .bind(user_id.value());
        let keys_res: Result<Vec<SqlxCryptoKey>, sqlx::Error> = query.fetch_all(&self.pool).await;
        if keys_res.is_err() {
            return match keys_res.err().unwrap() {
                sqlx::Error::RowNotFound => Ok(Vec::new()),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(keys_res.unwrap().iter().map(|key| key.clone().to_domain()).collect())
    }

    async fn find_by_id(&self, user_id: &UserId, id: &CryptoKeyId) -> Result<CryptoKey, DomainError> {
        let query = sqlx::query_as("SELECT id, name, payload, user_id, description, key_type, domain, status FROM cryptokeys WHERE id = $1 AND user_id = $2")
            .bind(id.value())
            .bind(user_id.value());
        let key_res: Result<SqlxCryptoKey, sqlx::Error> = query.fetch_one(&self.pool).await;
        if key_res.is_err() {
            return match key_res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CryptoKeyNotFound { id: id.value(), user_id: user_id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(key_res.unwrap().to_domain())
    }

    async fn create_one(&self, key: &CryptoKey) -> Result<(), DomainError> {
        let sql_user: SqlxCryptoKey = SqlxCryptoKey::from_domain(key);
        let res = sqlx::query("INSERT INTO cryptokeys (id, name, payload, user_id, description, key_type, domain, status) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&sql_user.id)
            .bind(&sql_user.name)
            .bind(&sql_user.payload)
            .bind(&sql_user.user_id)
            .bind(&sql_user.description)
            .bind(&sql_user.key_type)
            .bind(&sql_user.domain)
            .bind(&sql_user.status)
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() {
            return match res.err().unwrap() {
                sqlx::Error::Database(_) => Err(DomainError::CryptoKeyAlreadyExists { id: key.id.value(), user_id: key.user_id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(())
    }

    async fn update_one(&self, key: &CryptoKey) -> Result<(), DomainError> {
        let sql_key: SqlxCryptoKey = SqlxCryptoKey::from_domain(key);
        let res = sqlx::query("UPDATE cryptokeys SET name = $1, payload = $2, description = $3, key_type = $4, domain = $5, status = $6 WHERE id = $7 and user_id = $8")
            .bind(&sql_key.name)
            .bind(&sql_key.payload)
            .bind(&sql_key.description)
            .bind(&sql_key.key_type)
            .bind(&sql_key.domain)
            .bind(&sql_key.status)
            .bind(&sql_key.id)
            .bind(&sql_key.user_id)
            .fetch_optional(&self.pool)
            .await;

        if res.is_err() {
            return match res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CryptoKeyNotFound { id: key.id.value(), user_id: key.user_id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }

        Ok(())
    }

    async fn delete_one(&self, user_id: &UserId, id: &CryptoKeyId) -> Result<(), DomainError> {
        let res = sqlx::query("DELETE FROM kernel.cryptokeys WHERE id = $1 and user_id = $2")
            .bind(id.value())
            .bind(user_id.value())
            .fetch_optional(&self.pool)
            .await;
        if res.is_err() { // TODO: check sql error code or message
            return match res.err().unwrap() {
                sqlx::Error::RowNotFound => Err(DomainError::CryptoKeyNotFound { id: id.value(), user_id: user_id.value() }),
                err => {
                    error!("Error: {:?}", err);
                    Err(DomainError::Unknown)
                }
            }
        }
        Ok(())
    }
}

