use actix_web::web;
use kernel::users::domain::user_repository::UserRepository;

mod user_get_controller;
mod user_post_controller;

pub fn router<R: UserRepository>(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users")
            .configure(user_post_controller::route::<R>)
            .configure(user_get_controller::route::<R>),
    );
}
