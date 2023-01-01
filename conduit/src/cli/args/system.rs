/*
    Appellation: system <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use acme::net::WebBackend;
use clap::Args;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct System {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    up: bool,
}

impl System {
    pub fn new(up: bool) -> Self {
        Self { up }
    }
    fn commands(&self) -> AsyncResult<&Self> {
        if self.up {}
        Ok(self)
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::debug!("System processing...");
        if self.up {
            tracing::info!("Spawning the api...");
            let api = crate::api::new();
            api.serve().await?;
        }
        self.commands()?;
        Ok(self)
    }
}
