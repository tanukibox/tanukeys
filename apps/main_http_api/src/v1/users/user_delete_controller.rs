
use std::ops::Deref;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use kernel::users::{application::delete_one::user_deleter::UserDeleter, domain::{
        entities::user_id::UserId,
        user_repository::UserRepository,
    }};

pub fn route<R: UserRepository>(cfg: &mut ServiceConfig) {
    cfg.route("/{user_id}", web::delete().to(controller::<R>));
}

async fn controller<R: UserRepository>(
    user_id: web::Path<String>,
    deleter: web::Data<UserDeleter<R>>,
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
