use self::{metrics::MetricsConfig, router::RouterConfig};
use config::{ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use std::net::SocketAddr;

pub(super) mod metrics;
pub(super) mod router;

#[derive(Deserialize, Debug)]
/// Defines the structure of a config file. Extension can be
pub struct ServerConfig {
    /// listening address
    pub listen: SocketAddr,

    // pub routing: Option<RouterConfig>,
    /// routing configuration
    /// required because no other method is currently supported
    pub routing: RouterConfig,

    pub metrics: Option<MetricsConfig>,
}

impl ServerConfig {
    /// reads configuration from Config.toml
    /// (more file exts can be supported through config's features)
    pub fn read() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(File::new("Config.toml", FileFormat::Toml))
            .add_source(Environment::default().prefix("HOPPER"))
            .build()?
            .try_deserialize()
    }
}
