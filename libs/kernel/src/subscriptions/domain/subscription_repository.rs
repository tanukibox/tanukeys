use crate::subscriptions::domain::entities::subscription::Subscription;
use crate::shared::domain::entities::user_id::UserId;
use crate::shared::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait SubscriptionRepository: Send + Sync + 'static {
    async fn find_many(&self, user_id: &UserId) -> Result<Vec<Subscription>, DomainError>;
    async fn find_by_key_domain(&self, user_id: &UserId, key_domain: &str) -> Result<Subscription, DomainError>;
    async fn create_one(&self, subscription: &Subscription) -> Result<(), DomainError>;
    async fn update_one(&self, subscription: &Subscription) -> Result<(), DomainError>;
    async fn delete_one(&self, user_id: &UserId, key_domain: &str) -> Result<(), DomainError>;
} 