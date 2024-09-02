use actix_web::web;
use events::domain::event_bus::EventBus;
use kernel::users::domain::user_repository::UserRepository;

mod user_get_controller;
mod user_post_controller;
mod user_put_controller;
mod user_delete_controller;

pub fn router<R: UserRepository, E: EventBus>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users")
            .configure(user_post_controller::route::<R, E>)
            .configure(user_get_controller::route::<R>)
            .configure(user_put_controller::route::<R, E>)
            .configure(user_delete_controller::route::<R, E>),
    );
}
