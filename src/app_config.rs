use convert_case::{Case, Casing};
use serde::Deserialize;

use crate::errors::{AppError, Result};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "AppConfig::default_app_host")]
    pub tiny_url_host: String,
    #[serde(default = "AppConfig::default_app_port")]
    pub tiny_url_port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn init() -> Result<Self> {
        match envy::from_env::<AppConfig>() {
            Ok(config) => Ok(config),
            Err(error) => match error {
                // By using convert_case, we can display the field name in an expected manner
                envy::Error::MissingValue(field) => Err(AppError::MissingAppConfigField(format!(
                    "Missing env var: {}",
                    field.to_case(Case::UpperSnake)
                ))
                .into()),
                envy::Error::Custom(err_str) => {
                    Err(AppError::UnexpectedError(format!("Unexpected error: {}", err_str)).into())
                }
            },
        }
    }

    fn default_app_host() -> String {
        "127.0.0.1".to_string()
    }
    fn default_app_port() -> u16 {
        8080
    }
}
