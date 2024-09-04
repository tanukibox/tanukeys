use actix_web::web;
use actix_web::web::ServiceConfig;
use events::domain::event_bus::EventBus;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
use kernel::users::domain::user_repository::UserRepository;
mod crypto_key_post_controller;
mod crypto_key_get_controller;

pub fn router<R: CryptoKeyRepository, E: EventBus>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users/{user_id}/cryptokeys")
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/{crypto_key_id}", web::get().to(crypto_key_get_controller::controller::<R>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/", web::post().to(crypto_key_post_controller::controller::<R, E>)); })
    );
}
