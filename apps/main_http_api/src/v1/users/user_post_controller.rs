use std::ops::Deref;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use events::domain::event_bus::EventBus;
use kernel::users::{
    application::create_one::user_creator::UserCreator,
    domain::{
        entities::{user_id::UserId, user_name::UserName},
        user_repository::UserRepository,
    },
    infrastructure::dtos::json::user_dto::UserDto,
};

pub fn route<R: UserRepository, E: EventBus>(cfg: &mut ServiceConfig) {
    cfg.route("/", web::post().to(controller::<R, E>));
}

async fn controller<R: UserRepository, E: EventBus>(
    dto: web::Json<UserDto>,
    creator: web::Data<UserCreator<R, E>>,
) -> HttpResponse {
    let user_id = UserId::new(dto.id.clone());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let user_id = user_id.unwrap();
    let user_name = UserName::new(dto.name.clone());
    if user_name.is_err() {
        return HttpResponse::BadRequest().finish();
    }
    let user_name = user_name.unwrap();

    let res = creator.run(user_id, user_name).await;
    match res {
        Ok(_) => HttpResponse::Accepted().finish(),
        Err(err) => {
            if let Some(err) = err.deref().downcast_ref::<DomainError>() {
                match err.get_err_type() {
                    GeneralErrorTypes::ResourceNotFound => {
                        return HttpResponse::NotFound().body(err.message())
                    }
                    GeneralErrorTypes::ResourceAlreadyExists => {
                        return HttpResponse::Conflict().body(err.message())
                    }
                    GeneralErrorTypes::Other => {
                        return HttpResponse::InternalServerError().finish()
                    }
                };
            }
            HttpResponse::InternalServerError().finish()
        }
    }
}
