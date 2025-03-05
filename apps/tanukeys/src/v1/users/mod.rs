/// Users API module for the Tanukeys service.
/// 
/// This module provides endpoints for managing users in the system.
/// It includes operations for creating, retrieving, updating, and deleting users.
/// All endpoints require proper authentication and authorization.

use actix_web::web;
use actix_web::web::ServiceConfig;
use events::domain::event_bus::EventBus;
use kernel::users::domain::user_repository::UserRepository;

mod user_get_controller;
mod user_post_controller;
mod user_put_controller;
mod user_delete_controller;

/// Configures the users routes for the application.
/// 
/// This function sets up the following endpoints:
/// - POST /api/v1/users - Create a new user
/// - GET /api/v1/users/{user_id} - Get a specific user
/// - PUT /api/v1/users - Update a user
/// - DELETE /api/v1/users/{user_id} - Delete a specific user
/// 
/// # Arguments
/// 
/// * `cfg` - The service configuration to add the user routes to
/// * `R` - The repository type implementing UserRepository
/// * `E` - The event bus type implementing EventBus
pub fn router<R: UserRepository, E: EventBus>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/users")
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/",          web::post()  .to(user_post_controller::controller::<R, E>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/{user_id}", web::get()   .to(user_get_controller::controller::<R>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/",          web::put()   .to(user_put_controller::controller::<R, E>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/{user_id}", web::delete().to(user_delete_controller::controller::<R, E>)); })
    );
}
