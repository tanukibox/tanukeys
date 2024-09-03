use std::ops::Deref;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use events::domain::event_bus::EventBus;
use kernel::shared::domain::entities::user_id::UserId;
use kernel::users::{application::delete_one::user_deleter::UserDeleter, domain::user_repository::UserRepository};

pub fn route<R: UserRepository, E: EventBus>(cfg: &mut ServiceConfig) {
    cfg.route("/{user_id}", web::delete().to(controller::<R, E>));
}

pub(crate) async fn controller<R: UserRepository, E: EventBus>(
    user_id: web::Path<String>,
    deleter: web::Data<UserDeleter<R, E>>,
) -> HttpResponse {
    let user_id = UserId::new(user_id.clone());
    if user_id.is_err() {
        return HttpResponse::BadRequest().finish()
    }

    let res = deleter.run(user_id.unwrap()).await;
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
