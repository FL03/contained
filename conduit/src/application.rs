/*
    Appellation: application <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::Settings;
use scsys::prelude::{BoxResult, Configurable, Context};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Application {
    pub cnf: Settings,
    pub ctx: Context<Settings>,
}

impl Application {
    pub fn new(cnf: Settings, ctx: Context<Settings>) -> Self {
        Self { cnf, ctx }
    }
    // Initialize application logging
    pub fn with_logging(&mut self) -> &Self {
        self.cnf.logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::debug!("Success: Initialized the logging protocols");
        self
    }

    pub async fn quickstart(&mut self) -> BoxResult<&Self> {
        self.with_logging();
        tracing::info!("Startup: Application initializing...");

        Ok(self)
    }
}

impl std::convert::From<Settings> for Application {
    fn from(data: Settings) -> Self {
        Self::new(data.clone(), Context::new(data))
    }
}

impl std::convert::From<Context<Settings>> for Application {
    fn from(data: Context<Settings>) -> Self {
        Self::new(data.clone().settings, data)
    }
}

impl Configurable for Application {
    type Settings = Settings;

    fn settings(&self) -> &Self::Settings {
        &self.ctx.settings
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
