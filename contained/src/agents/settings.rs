/*
    Appellation: settings <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Each agent has a set of settings that are used to configure the agent.
*/
use config::{Config, Environment};
use scsys::prelude::{try_collect_config_files, ConfigResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Settings {
    pub mode: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            mode: "production".to_string(),
        }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Self::builder().set_default("mode", "production")?;

        if let Ok(mode) = std::env::var("AGENT_MODE") {
            builder = builder.set_override("mode", mode)?;
        };
        // Add in related environment variables
        builder = builder.add_source(Environment::default().separator("__").prefix("AGENT"));
        // Try gathering valid configuration files...
        if let Ok(files) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(files);
        }
        builder.build()?.try_deserialize()
    }
    pub fn builder() -> config::ConfigBuilder<config::builder::DefaultState> {
        Config::builder()
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::build() {
            Ok(cnf) => cnf,
            Err(_) => Self::new(),
        }
    }
}
