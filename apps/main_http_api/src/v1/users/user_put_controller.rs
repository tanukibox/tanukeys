
use actix_web::{
    web,
    HttpResponse,
};
use events::domain::event_bus::EventBus;
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;
use kernel::users::{
    application::update_one::user_updater::UserUpdater, domain::{
        entities::user_name::UserName,
        user_repository::UserRepository,
    }, infrastructure::dtos::json::user_dto::UserDto
};

pub(crate) async fn controller<R: UserRepository, E: EventBus>(
    dto: web::Json<UserDto>,
    updater: web::Data<UserUpdater<R, E>>,
) -> HttpResponse {
    let user_id = UserId::new(dto.id.clone());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish()
    }
    let user_name = UserName::new(dto.name.clone());
    if user_name.is_err() {
        return HttpResponse::BadRequest().finish()
    }

    let res = updater.run(user_id.unwrap(), user_name.unwrap()).await;
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
