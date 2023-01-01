/*
    Appellation: accounts <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AccountArgs {
    #[clap(long, short, value_parser)]
    address: Option<String>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    login: bool,
}

impl AccountArgs {
    pub fn new(address: Option<String>, login: bool) -> Self {
        Self { address, login }
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
