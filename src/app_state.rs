use crate::{app_config::AppConfig, db::DB};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DB,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self {
        Self {
            config,
            db: DB::new().await,
        }
    }
}
