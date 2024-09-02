use crate::shared::domain::entities::user_id::UserId;
use crate::users::domain::entities::user_name::UserName;
use events::domain::domain_event::DomainEvent;


pub struct UserUpdatedEvent {
    pub id: String,
    pub user_id: UserId,
    pub user_name: UserName,
    pub old_user_name: UserName,
    pub occurred_on: String,
}

impl UserUpdatedEvent {
    pub fn new(user_id: UserId, user_name: UserName, old_user_name: UserName) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let occurred_on = chrono::Utc::now().to_rfc3339();
        Self { id, user_id, user_name, old_user_name, occurred_on }
    }

    pub fn new_shared(user_id: UserId, user_name: UserName, old_user_name: UserName) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self::new(user_id, user_name, old_user_name))
    }
}

impl DomainEvent for UserUpdatedEvent {
    fn event_type(&self) -> String {
        "tanukeys.kernel.users.updated@1.0.0".to_string()
    }
}