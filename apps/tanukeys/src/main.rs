use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use actix_web::{web::Data, App, HttpServer};

use events::infrastructure::inmemory::inmemory_event_bus::InMemoryEventBus;
use kernel::{crypto_keys::application::find_many_by_user::crypto_keys_by_user_finder::CryptoKeysByUserFinder, users::{
    application::{create_one::user_creator::UserCreator, delete_one::user_deleter::UserDeleter, find_one::user_finder::UserFinder, update_one::user_updater::UserUpdater},
    infrastructure::sqlx::sqlx_postgres_user_repository::SqlxPostgresUserRepository,
}};
use v1::health::health_controller;

use tracing::{self as logger};
use kernel::crypto_keys::application::create_one::crypto_key_creator::CryptoKeyCreator;
use kernel::crypto_keys::application::find_one::crypto_key_finder::CryptoKeyFinder;
use kernel::crypto_keys::infrastructure::sqlx::sqlx_postgres_crypto_key_repository::SqlxPostgresCryptoKeyRepository;

pub mod v1;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("./apps/tanukeys/.env").ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        //.pretty()
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc3339())
        .init();

    let host = "0.0.0.0";
    let port = std::env::var("PORT").expect("PORT must be set");
    let address = format!("{}:{}", host, port);
    logger::info!("Starting server at http://{}:{}/.", host, port);

    let thread_counter = Arc::new(AtomicU16::new(0));

    let event_bus = InMemoryEventBus::new();
    let event_bus_ref = Arc::new(event_bus);

    let user_repository = SqlxPostgresUserRepository::from_env().await;
    let user_repository_ref = Arc::new(user_repository);

    let user_finder = UserFinder::new(user_repository_ref.clone());
    let user_finder_ref = Data::new(user_finder);

    let user_creator = UserCreator::new(user_repository_ref.clone(), event_bus_ref.clone());
    let user_creator_ref = Data::new(user_creator);

    let user_updater = UserUpdater::new(user_repository_ref.clone(), event_bus_ref.clone());
    let user_updater_ref = Data::new(user_updater);

    let user_deleter = UserDeleter::new(user_repository_ref.clone(), event_bus_ref.clone());
    let user_deleter_ref = Data::new(user_deleter);

    let crypto_key_repository = SqlxPostgresCryptoKeyRepository::from_env().await;
    let crypto_key_repository_ref = Arc::new(crypto_key_repository);

    let crypto_key_finder = CryptoKeyFinder::new(crypto_key_repository_ref.clone());
    let crypto_key_finder_ref = Data::new(crypto_key_finder);

    let crypto_keys_by_user_finder = CryptoKeysByUserFinder::new(crypto_key_repository_ref.clone());
    let crypto_keys_by_user_finder_ref = Data::new(crypto_keys_by_user_finder);

    let crypto_key_creator = CryptoKeyCreator::new(crypto_key_repository_ref.clone(), event_bus_ref.clone());
    let crypto_key_creator_ref = Data::new(crypto_key_creator);

    HttpServer::new(move || {
        let thread_counter = thread_counter.fetch_add(1, Ordering::SeqCst);
        logger::info!("Thread {} started.", thread_counter);

        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            
            .app_data(user_finder_ref.clone())
            .app_data(user_creator_ref.clone())
            .app_data(user_updater_ref.clone())
            .app_data(user_deleter_ref.clone())

            .app_data(crypto_key_finder_ref.clone())
            .app_data(crypto_keys_by_user_finder_ref.clone())
            .app_data(crypto_key_creator_ref.clone())

            .configure(v1::users::router::<SqlxPostgresUserRepository, InMemoryEventBus>)
            .configure(v1::crypto_keys::router::<SqlxPostgresCryptoKeyRepository, InMemoryEventBus>)
            .configure(health_controller::router)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!("🔥🔥🔥 Can not bind to address: {} 🔥🔥🔥", err);
    })
    .run()
    .await
}
