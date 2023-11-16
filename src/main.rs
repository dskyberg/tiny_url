use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

mod app_config;
mod app_state;
mod db;
mod errors;
mod handlers;
mod models;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file, if exists
    dotenv::dotenv().ok();

    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load applicagtion config from ENV
    let config = app_config::AppConfig::init().expect("Well, shit");

    // Hand a copy of the config to the app_state.
    let app_state = app_state::AppState::new(config.clone()).await;

    log::info!("database url: {}", &app_state.config.database_url);
    log::info!(
        "🚀 version {}: running on http://localhost:{}",
        &app_state.config.package.version,
        app_state.config.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(handlers::config)
            .wrap(Logger::default())
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
