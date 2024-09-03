use std::ops::Deref;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use kernel::shared::domain::entities::user_id::UserId;
use kernel::users::{
    application::find_one::user_finder::UserFinder,
    domain::user_repository::UserRepository,
    infrastructure::dtos::json::user_dto::parse_to_dto,
};

pub fn route<R: UserRepository>(cfg: &mut ServiceConfig) {
    cfg.route("/{user_id}", web::get().to(controller::<R>));
}

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
