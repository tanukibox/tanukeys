use std::ops::Deref;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use events::domain::event_bus::EventBus;
use kernel::shared::domain::entities::user_id::UserId;
use kernel::users::{
    application::update_one::user_updater::UserUpdater, domain::{
        entities::user_name::UserName,
        user_repository::UserRepository,
    }, infrastructure::dtos::json::user_dto::UserDto
};

pub fn route<R: UserRepository, E: EventBus>(cfg: &mut ServiceConfig) {
    cfg.route("/", web::put().to(controller::<R, E>));
}

async fn controller<R: UserRepository, E: EventBus>(
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
