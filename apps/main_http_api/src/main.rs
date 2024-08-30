use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use actix_web::{web::Data, App, HttpServer};

use kernel::users::{
    application::{create_one::user_creator::UserCreator, find_one::user_finder::UserFinder, update_one::user_updater::UserUpdater},
    infrastructure::sqlx::sqlx_postgres_user_repository::SqlxPostgresUserRepository,
};
use v1::health::health_controller;

use tracing::{self as logger};

pub mod v1;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("./apps/main_http_api/.env").ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        //.pretty()
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc3339())
        .init();

    let host = "127.0.0.1";
    let port = std::env::var("PORT").expect("PORT must be set");
    let address = format!("{}:{}", host, port);
    logger::info!("Starting server at http://{}:{}/.", host, port);

    let thread_counter = Arc::new(AtomicU16::new(0));

    let user_repository = SqlxPostgresUserRepository::from_env().await;
    let user_repository_ref = Arc::new(user_repository);

    let user_finder = UserFinder::new(user_repository_ref.clone());
    let user_finder_ref = Data::new(user_finder);

    let user_creator = UserCreator::new(user_repository_ref.clone());
    let user_creator_ref = Data::new(user_creator);

    let user_updater = UserUpdater::new(user_repository_ref.clone());
    let user_updater_ref = Data::new(user_updater);

    HttpServer::new(move || {
        let thread_counter = thread_counter.fetch_add(1, Ordering::SeqCst);
        logger::info!("Thread {} started.", thread_counter);
        App::new()
            .app_data(user_finder_ref.clone())
            .app_data(user_creator_ref.clone())
            .app_data(user_updater_ref.clone())
            .configure(v1::users::router::<SqlxPostgresUserRepository>)
            .configure(health_controller::router)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!("🔥🔥🔥 Can not bind to address: {} 🔥🔥🔥", err);
    })
    .run()
    .await
}
