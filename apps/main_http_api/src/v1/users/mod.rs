use actix_web::web;
use actix_web::web::ServiceConfig;
use events::domain::event_bus::EventBus;
use kernel::users::domain::user_repository::UserRepository;

mod user_get_controller;
mod user_post_controller;
mod user_put_controller;
mod user_delete_controller;

pub fn router<R: UserRepository, E: EventBus>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users")
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/",          web::post().to(user_post_controller::controller::<R, E>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/{user_id}", web::get().to(user_get_controller::controller::<R>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/",          web::post().to(user_put_controller::controller::<R, E>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/{user_id}", web::post().to(user_delete_controller::controller::<R, E>)); })
    );
}
