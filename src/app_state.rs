use crate::{app_config::AppConfig, repository::PostgresUrlRepository};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    //TODO: Convert this to Box<dyn UrlRepository>
    pub db: PostgresUrlRepository,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self {
        Self {
            config,
            db: PostgresUrlRepository::new().await,
        }
    }
}
