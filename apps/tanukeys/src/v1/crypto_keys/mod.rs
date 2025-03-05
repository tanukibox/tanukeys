/// Crypto Keys API module for the Tanukeys service.
/// 
/// This module provides endpoints for managing cryptographic keys in the system.
/// It includes operations for creating, retrieving, listing, and deleting crypto keys.
/// All endpoints require proper authentication and authorization.

use actix_web::web;
use actix_web::web::ServiceConfig;
use events::domain::event_bus::EventBus;
use kernel::crypto_keys::domain::crypto_key_repository::CryptoKeyRepository;
mod crypto_key_post_controller;
mod crypto_key_get_controller;
mod crypto_keys_get_controller;
mod crypto_key_delete_controller;

/// Configures the crypto keys routes for the application.
/// 
/// This function sets up the following endpoints:
/// - GET /api/v1/cryptokeys - List all crypto keys for a user
/// - GET /api/v1/cryptokeys/{crypto_key_id} - Get a specific crypto key
/// - POST /api/v1/cryptokeys - Create a new crypto key
/// - DELETE /api/v1/cryptokeys/{crypto_key_id} - Delete a specific crypto key
/// 
/// # Arguments
/// 
/// * `cfg` - The service configuration to add the crypto key routes to
/// * `R` - The repository type implementing CryptoKeyRepository
/// * `E` - The event bus type implementing EventBus
pub fn router<R: CryptoKeyRepository, E: EventBus>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/cryptokeys")
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/", web::get().to(crypto_keys_get_controller::controller::<R>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/{crypto_key_id}", web::get().to(crypto_key_get_controller::controller::<R>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/", web::post().to(crypto_key_post_controller::controller::<R, E>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/{crypto_key_id}", web::delete().to(crypto_key_delete_controller::controller::<R, E>)); })
    );
}
