/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use config::{Config, Environment};
use decanter::prelude::{hasher, Hashable, H256};
use scsys::prelude::{try_collect_config_files, ConfigResult, SerdeDisplay};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, SerdeDisplay, Serialize,
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

impl Hashable for Logger {
    fn hash(&self) -> H256 {
        hasher(self).into()
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
    Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, SerdeDisplay, Serialize,
)]
pub struct Settings {
    pub logger: Logger,
    pub mode: String,
}

impl Settings {
    pub fn new(mode: Option<String>) -> Self {
        Self {
            logger: Default::default(),
            mode: mode.unwrap_or_else(|| String::from("production")),
        }
    }
    pub fn builder() -> config::ConfigBuilder<config::builder::DefaultState> {
        Config::builder()
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Self::builder()
            .set_default("mode", "production")?
            .set_default("logger.level", "info")?;

        if let Ok(log) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", log)?;
        };
        if let Ok(port) = std::env::var("SERVER_PORT") {
            builder = builder.set_override("server.port", port)?;
        };
        // Add in related environment variables
        builder = builder.add_source(
            Environment::default()
                .separator("__")
                .prefix(env!("CARGO_PKG_NAME").to_ascii_uppercase().as_str()),
        );
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

impl Hashable for Settings {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(None);
        Self::build().unwrap_or(d)
    }
}
