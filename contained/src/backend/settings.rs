/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use config::{Config, Environment};
use decanter::prelude::Hashable;
use scsys::prelude::{try_collect_config_files, ConfigResult, SerdeDisplay};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct Settings {
    pub cluster: ClusterConfig,
    pub logger: Logger,
    pub mode: String,
}

impl Settings {
    pub fn new(mode: Option<String>) -> Self {
        Self {
            cluster: ClusterConfig::default(),
            logger: Default::default(),
            mode: mode.unwrap_or_else(|| String::from("production")),
        }
    }
    pub fn builder() -> config::ConfigBuilder<config::builder::DefaultState> {
        Config::builder()
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = {
            Self::builder()
                .set_default("cluster.addr", crate::net::DEFAULT_MULTIADDR)?
                .set_default("logger.level", "info")?
                .set_default("mode", "production")?
        };
        // Try loading in environment variables; prefixed with the package name and separated by "__"
        builder = builder.add_source(
            Environment::default()
                .separator("__")
                .prefix(env!("CARGO_PKG_NAME").to_ascii_uppercase().as_str()),
        );
        // Try overriding configuration values with specific environment variables...
        if let Ok(port) = std::env::var("CLUSTER_ADDR") {
            builder = builder.set_override("cluster.addr", port)?;
        };
        if let Ok(log) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", log)?;
        };
        // Try gathering valid configuration files...
        if let Ok(files) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(files);
        }
        builder.build()?.try_deserialize()
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(None);
        Self::build().unwrap_or(d)
    }
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: tracing::Level::INFO.to_string(),
        }
    }
    pub fn set_level(mut self, level: impl ToString) {
        self.level = level.to_string();
    }
    pub fn setup_env(mut self, level: Option<&str>) -> Self {
        let key = level.unwrap_or("RUST_LOG");
        if let Some(v) = std::env::var_os(key) {
            self.level = v.into_string().expect("Failed to convert into string...");
        } else {
            std::env::set_var(key, self.level.clone());
        }
        self
    }
    pub fn init_tracing(self) {
        tracing_subscriber::fmt::init();
        tracing::debug!("Success: tracing layer initialized...");
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl From<tracing::Level> for Logger {
    fn from(level: tracing::Level) -> Self {
        Self {
            level: level.to_string(),
        }
    }
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct ClusterConfig {
    pub addr: String,
    pub seed: Option<u8>,
}

impl ClusterConfig {
    pub fn new() -> Self {
        Self {
            addr: crate::net::DEFAULT_MULTIADDR.to_string(),
            seed: None,
        }
    }
    pub fn setup_env(
        mut self,
        addr: Option<&str>,
        seed: Option<&str>,
    ) -> crate::prelude::Resultant<Self> {
        let addr_key = addr.unwrap_or("CLUSTER_ADDR");
        let seed_key = seed.unwrap_or("CLUSTER_SEED");
        if let Some(v) = std::env::var_os(addr_key) {
            self.addr = v.into_string().unwrap();
        } else {
            std::env::set_var(addr_key, self.addr.clone());
        }
        if let Some(v) = std::env::var_os(seed_key) {
            let seed = v
                .into_string()
                .unwrap()
                .parse()
                .expect("Failed to parse the seed value...");
            self.seed = Some(seed);
        } else {
            let seed = self.seed.unwrap_or(0);
            std::env::set_var(seed_key, seed.to_string());
        }
        Ok(self)
    }
    pub fn set_addr(mut self, addr: impl ToString) {
        self.addr = addr.to_string();
    }
    pub fn set_seed(mut self, seed: u8) {
        self.seed = Some(seed);
    }
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            addr: crate::net::DEFAULT_MULTIADDR.to_string(),
            seed: None,
        }
    }
}
