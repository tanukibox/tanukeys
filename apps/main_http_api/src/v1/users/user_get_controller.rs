
use actix_web::{
    web,
    HttpResponse,
};
use kernel::shared::domain::entities::user_id::UserId;
use kernel::shared::domain::errors::DomainError;
use kernel::users::{
    application::find_one::user_finder::UserFinder,
    domain::user_repository::UserRepository,
    infrastructure::dtos::json::user_dto::parse_to_dto,
};

pub(crate) async fn controller<R: UserRepository>(
    user_id: web::Path<String>,
    finder: web::Data<UserFinder<R>>,
) -> HttpResponse {
    let user_id = UserId::new(user_id.parse().unwrap());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish()
    }
    let res = finder.run(user_id.unwrap()).await;
    match res {
        Ok(user) => {
            let dto = parse_to_dto(&user);
            HttpResponse::Ok().json(dto)
        }
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
