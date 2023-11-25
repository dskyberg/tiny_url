use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::repository::{PostgresUrlRepository, UrlRepository};
use crate::service::UrlServiceImpl;

mod app_config;
mod app_state;
mod errors;
mod handlers;
mod model;
mod repository;
mod service;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file, if exists
    dotenv::dotenv().ok();

    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load applicagtion config from ENV
    let config = app_config::AppConfig::init().expect("Well, shit");
    let db = PostgresUrlRepository::new().await;
    let service = Box::new(UrlServiceImpl::new(db));
    // Hand a copy of the config to the app_state.
    let app_state = app_state::AppState::new(config.clone(), service).await;

    log::info!("database url: {}", &app_state.config.database_url);
    log::info!(
        "🚀 version {}: running on http://localhost:{}",
        &app_state.config.package.version,
        app_state.config.port
    );

    #[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::get_url_handler,
        ),
        components(
            schemas(model::TinyUrl, model::UrlRequest, model::ErrorResponse, model::MessageResponse, model::ListUrlsResponse)
        ),
        tags(
            (name = "tiny_url", description = "Url Minifier with sanoids.")
        )
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(handlers::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .wrap(Logger::default())
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
