use std::env::var;

use crate::errors::{AppError, Result};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub tiny_url_host: String,
    pub tiny_url_port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn init() -> Result<Self> {
        let tiny_url_host = env_str("TINY_URL_HOST", Some("127.0.0.1")).map_err(|e| {
            log::error!("Error reading env var {}", "TINY_URL_HOST");
            e
        })?;

        // Default port is 8080. Override with HTTP_PORT env var
        let tiny_url_port = env_u16("TINY_URL_PORT", Some(8080)).map_err(|e| {
            log::error!("Error reading env var {}", "TINY_URL_PORT");
            e
        })?;

        let database_url = env_str("DATABASE_URL", None).map_err(|e| {
            log::error!("Error reading env var {}", "DATABASE_URL");
            e
        })?;

        Ok(Self {
            tiny_url_host,
            tiny_url_port,
            database_url,
        })
    }
}

fn env_str(key: &str, default: Option<&str>) -> Result<String> {
    match default {
        Some(val) => Ok(var(key).unwrap_or(val.to_string())),
        None => Ok(var(key).map_err(AppError::EnvNotFound)?),
    }
}

fn env_u16(key: &str, default: Option<u16>) -> Result<u16> {
    let env_val = var(key);
    match env_val {
        Ok(s) => Ok(s.parse::<u16>()?),
        Err(e) => Ok(default.ok_or(AppError::EnvNotFound(e))?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16_with_option() {
        std::env::set_var("DUMMY_VAR", "8080");
        let result = env_u16("DUMMY_VAR", Some(8080));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8080);
    }
}
