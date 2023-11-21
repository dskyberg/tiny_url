use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

use crate::errors::Result;

/// Grab interesting bits from Cargo.toml
#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
}
/// Default for Package
/// This is needed so that the AppConfig can default.
/// The alternative is to use Option<Package>.
impl Default for Package {
    fn default() -> Self {
        Self {
            name: "Cargo.toml does not have a name attribute!".into(),
            version: "Cargo.toml does not have a version attribute!".into(),
            description: None,
            authors: vec![],
        }
    }
}

/// AppConfig uses [figment] to marshall attribute values from multiple
/// sources.  To extend, simply add more attributes.  Make sure to extend [AppConfigBuilder] appropriately as well.
#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub package: Package,
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

/// For
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            package: Package::default(),
            host: "127.0.0.1".into(),
            port: 8080,
            database_url: "postgresql://postgres:postgres@localhost:5432/tiny_url_port".into(),
        }
    }
}

impl AppConfig {
    pub fn init() -> Result<Self> {
        let acb = AppConfigBuilder::init()?;
        Self::try_from(acb)
    }
}

impl TryFrom<AppConfigBuilder> for AppConfig {
    type Error = Box<dyn std::error::Error>;
    fn try_from(acb: AppConfigBuilder) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut config = AppConfig::default();
        if let Some(package) = acb.package {
            config.package = package;
        }
        if let Some(host) = acb.host {
            config.host = host;
        }
        if let Some(port) = acb.port {
            config.port = port;
        }
        if let Some(databbase_url) = acb.database_url {
            config.database_url = databbase_url;
        }
        Ok(config)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigBuilder {
    pub package: Option<Package>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database_url: Option<String>,
}

impl AppConfigBuilder {
    pub fn init() -> Result<Self> {
        let config: Self = Figment::new()
            .merge(Toml::file("Cargo.toml"))
            .merge(Toml::file("App.toml"))
            .merge(Env::prefixed("TINY_URL_"))
            .merge(Env::raw().only(&["DATABASE_URL"]))
            .extract()?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("TINY_URL_HOST", "bar");
            jail.set_env("BAZ", "put");

            let config = AppConfig::init().expect("well, shit");
            println!("{config:#?}");
            Ok(())
        });
    }
}
