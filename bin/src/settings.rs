/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::config::{Config, Environment};
use scsys::prelude::{try_collect_config_files, ConfigResult, Configurable, Logger, Server};
use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub logger: Logger,
    pub mode: String,
    pub name: String,
    pub server: Server,
}

impl Settings {
    pub fn new(logger: Option<Logger>, mode: Option<String>, name: Option<String>, server: Option<Server>) -> Self {
        Self {
            logger: logger.unwrap_or_default(),
            mode: mode.unwrap_or_else(|| String::from("production")),
            name: name.unwrap_or_else(|| String::from(env!("CARGO_PKG_NAME"))),
            server: server.unwrap_or(Server::new("0.0.0.0".to_string(), 8080)),
        }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("mode", "production")?
            .set_default("name", env!("CARGO_PKG_NAME"))?
            .set_default("logger.level", "info")?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?;

        if let Ok(files) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(files);
        }
        if let Ok(log) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", log)?;
        };
        if let Ok(port) = std::env::var("SERVER_PORT") {
            builder = builder.set_override("server.port", port)?;
        };
        builder.build()?.try_deserialize()
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(None, None, None, None);
        Self::build().unwrap_or(d)
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
