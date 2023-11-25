use crate::{app_config::AppConfig, service::UrlService};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    //TODO: Convert this to Box<dyn UrlRepository>
    pub service: Box<dyn UrlService>,
}

impl AppState {
    pub async fn new(config: AppConfig, service: Box<dyn UrlService>) -> Self {
        Self { config, service }
    }
}
