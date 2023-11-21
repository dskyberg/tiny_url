use crate::{
    app_config::AppConfig,
    repository::{PostgresUrlRepository, UrlRepository},
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: Box<dyn UrlRepository>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self {
        Self {
            config,
            db: Box::new(PostgresUrlRepository::new().await),
        }
    }
}
