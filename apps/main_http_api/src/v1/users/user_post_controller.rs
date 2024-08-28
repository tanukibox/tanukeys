use std::ops::Deref;

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use domain_errors::domain_error::{DomainError, GeneralErrorTypes};
use kernel::users::{
    application::create_one::user_creator::UserCreator,
    domain::{
        entities::{user_id::UserId, user_name::UserName},
        user_repository::UserRepository,
    },
    infrastructure::dtos::json::user_dto::UserDto,
};

pub fn route<R: UserRepository>(cfg: &mut ServiceConfig) {
    cfg.route("/", web::post().to(controller::<R>));
}

async fn controller<R: UserRepository>(
    dto: web::Json<UserDto>,
    creator: web::Data<UserCreator<R>>,
) -> HttpResponse {
    let user_id = UserId::new(dto.id.clone());
    let user_name = UserName::new(dto.name.clone());

    let res = creator.run(user_id, user_name).await;
    match res {
        Ok(_) => HttpResponse::Created().finish(),
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
