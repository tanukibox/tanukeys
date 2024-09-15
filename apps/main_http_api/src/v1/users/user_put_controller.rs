
use actix_web::{
    web, HttpRequest, HttpResponse
};
use events::domain::event_bus::EventBus;
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;
use kernel::users::infrastructure::dtos::json::user_dto::parse_to_domain;
use kernel::users::{
    application::update_one::user_updater::UserUpdater, domain::user_repository::UserRepository, infrastructure::dtos::json::user_dto::UserDto
};

pub(crate) async fn controller<R: UserRepository, E: EventBus>(
    dto: web::Json<UserDto>,
    req: HttpRequest,
    updater: web::Data<UserUpdater<R, E>>,
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

    let user = parse_to_domain(&dto);
    if user.is_err() {
        return HttpResponse::BadRequest().body(user.err().unwrap().to_string());
    }
    let user = user.unwrap();

    let res = updater.run(user.id, user.name, user.bio, auth_user).await;
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
                DomainError::UserNotAuthorized { user_id } => {
                    return HttpResponse::Unauthorized().body(format!("User with id <{}>, not authorized.", user_id))
                },
                _ => return HttpResponse::InternalServerError().finish()
            }
        }
    }
}
