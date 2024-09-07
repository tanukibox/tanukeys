
use actix_web::{
    web, HttpRequest, HttpResponse
};
use events::domain::event_bus::EventBus;
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;
use kernel::users::{
    application::create_one::user_creator::UserCreator,
    domain::{
        entities::user_name::UserName,
        user_repository::UserRepository,
    },
    infrastructure::dtos::json::user_dto::UserDto,
};

pub(crate) async fn controller<R: UserRepository, E: EventBus>(
    dto: web::Json<UserDto>,
    req: HttpRequest,
    creator: web::Data<UserCreator<R, E>>,
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
    let user_id = UserId::new(dto.id.clone());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish()
    }
    let user_name = UserName::new(dto.name.clone());
    if user_name.is_err() {
        return HttpResponse::BadRequest().finish()
    }

    let res = creator.run(user_id.unwrap(), user_name.unwrap(), auth_user).await;
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
