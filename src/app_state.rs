use crate::{app_config::AppConfig, repository::UrlRepository};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    //TODO: Convert this to Box<dyn UrlRepository>
    pub db: Box<dyn UrlRepository>,
}

impl AppState {
    pub async fn new(config: AppConfig, db: Box<dyn UrlRepository>) -> Self {
        Self { config, db }
    }
}
