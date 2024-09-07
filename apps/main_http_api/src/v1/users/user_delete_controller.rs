
use actix_web::{
    web, HttpRequest, HttpResponse
};
use events::domain::event_bus::EventBus;
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;
use kernel::users::{application::delete_one::user_deleter::UserDeleter, domain::user_repository::UserRepository};

pub(crate) async fn controller<R: UserRepository, E: EventBus>(
    user_id: web::Path<String>,
    req: HttpRequest,
    deleter: web::Data<UserDeleter<R, E>>,
) -> HttpResponse {
    let auth_user = req.headers().get("Authorization");
    if auth_user.is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let auth_user = UserId::new(auth_user.unwrap().to_str().unwrap().to_string());
    if auth_user.is_err() {
        return HttpResponse::Unauthorized().finish();
    }
    let auth_user = auth_user.unwrap();
    let user_id = UserId::new(user_id.clone());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let user_id = user_id.unwrap();

    let res = deleter.run(user_id, auth_user).await;
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
