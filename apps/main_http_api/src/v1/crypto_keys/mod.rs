pub mod crypto_key_post_controller;

use actix_web::web;
use events::domain::event_bus::EventBus;
use kernel::users::domain::user_repository::UserRepository;

pub fn router<R: UserRepository, E: EventBus>(cfg: &mut web::ServiceConfig) {

}
