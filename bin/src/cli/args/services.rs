/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Services {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    up: bool,
}

impl Services {
    pub fn new(up: bool) -> Self {
        Self { up }
    }
    fn commands(&self) -> AsyncResult<&Self> {
        Ok(self)
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::debug!("System processing...");

        self.commands()?;
        Ok(self)
    }
}
