/*
    Appellation: setup <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use scsys::BoxResult;
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
    fn commands(&self) -> BoxResult<&Self> {
        Ok(self)
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Setting up the workspace...");
        self.commands()?;
        Ok(self)
    }
}
