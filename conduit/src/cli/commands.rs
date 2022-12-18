/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Subcommand;
use scsys::BoxResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Connect {
        #[clap(long, short, value_parser)]
        address: String,
    },
    System {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        up: bool,
    },
}

impl Commands {
    pub fn handle(&self) -> tokio::task::JoinHandle<Arc<Self>> {
        let cmds = Arc::new(self.clone());
        tokio::spawn(async move {
            cmds.handler().ok().unwrap();
            println!("{:?}", cmds.clone());
            cmds
        })
    }
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");

        match self.clone() {
            Commands::Connect { address } => {
                println!("{:?}", address);
            }
            Commands::System { up } => {
                if up {
                    tracing::info!("Turning on the application subsystems...");
                }
            }
        }
        Ok(self)
    }
}
