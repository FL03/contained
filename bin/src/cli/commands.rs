/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::args::{System};
use clap::Subcommand;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, Subcommand)]
pub enum Commands {
    System(System),
}

impl Commands {
    pub async fn handler(&self, ctx: crate::Context) -> AsyncResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::System(system) => {
                system.handler(ctx).await?;
            }
        };
        Ok(self)
    }
}
