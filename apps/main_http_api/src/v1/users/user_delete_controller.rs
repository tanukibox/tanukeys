
use actix_web::{
    web,
    HttpResponse,
};
use events::domain::event_bus::EventBus;
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;
use kernel::users::{application::delete_one::user_deleter::UserDeleter, domain::user_repository::UserRepository};

pub(crate) async fn controller<R: UserRepository, E: EventBus>(
    user_id: web::Path<String>,
    deleter: web::Data<UserDeleter<R, E>>,
) -> HttpResponse {
    let user_id = UserId::new(user_id.clone());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let user_id = user_id.unwrap();

    let res = deleter.run(user_id).await;
    match res {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(err) => {
            match err {
                DomainError::UserAlreadyExists { user_id } => {
                    return HttpResponse::Conflict().body(format!("User with id <{}>, already exists.", user_id))
                },
                DomainError::UserNotFound { user_id } => {
                    return HttpResponse::NotFound().body(format!("User with id <{}>, not found.", user_id))
                },
                _ => return HttpResponse::InternalServerError().finish()
            }
        }
    }
}
