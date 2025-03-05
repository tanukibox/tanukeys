/// Health check module for the Tanukeys API.
/// 
/// This module provides endpoints to check the health status of the API.
/// It includes a simple health check endpoint that returns a 200 OK response
/// when the service is up and running.

use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use tracing::{self as logger};

/// Configures the health check routes for the application.
/// 
/// This function sets up the health check endpoint at `/health`.
/// The endpoint is accessible via GET requests and returns a 200 OK response
/// when the service is healthy.
/// 
/// # Arguments
/// 
/// * `cfg` - The service configuration to add the health check route to
pub fn router(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(controller));
}

/// Handles health check requests.
/// 
/// This function processes GET requests to the `/health` endpoint.
/// It logs the request and returns a 200 OK response to indicate
/// that the service is up and running.
/// 
/// # Returns
/// 
/// An `HttpResponse` with status code 200 OK
async fn controller() -> HttpResponse {
    logger::debug!("Health check endpoint called.");
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod unit_tests {
    use actix_web::http::StatusCode;

    use super::controller;

    #[actix_rt::test]
    async fn health_check_works() {
        let res = controller().await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
    }
}

#[cfg(test)]
mod integration_tests {
    use actix_web::{
        http::StatusCode,
        test::{call_service, init_service, TestRequest},
        App,
    };

    #[actix_rt::test]
    async fn health_check_works() {
        let app = App::new().configure(super::router);
        let app = init_service(app).await;
        let req = TestRequest::get().uri("/health").to_request();
        let res = call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
    }
}
