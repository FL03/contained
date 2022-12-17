/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{commands::*, context::*};

pub(crate) mod commands;

pub fn new() -> Cli {
    Cli::default()
}

pub(crate) mod context {
    use super::Commands;
    use crate::states::{State, States};
    use clap::Parser;
    use scsys::BoxResult;
    use serde::{Deserialize, Serialize};
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Parser, PartialEq, Serialize)]
    #[clap(about, author, version)]
    #[clap(long_about = None)]
    pub struct Cli {
        #[clap(subcommand)]
        pub command: Option<Commands>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub debug: bool,
        #[arg(action = clap::ArgAction::SetTrue, long)]
        pub detached: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub update: bool,
    }

    impl Cli {
        pub fn new() -> Self {
            Self::parse()
        }
        pub fn locked(&self) -> Arc<Mutex<Self>> {
            Arc::from(Mutex::new(self.clone()))
        }
        pub async fn handle(&self) -> std::thread::JoinHandle<Arc<Mutex<Self>>> {
            let cli = self.locked();
            let handle = std::thread::spawn(move || {
                    cli.clone()
                }
            );
            handle
        }
        pub async fn handler(&self, state: &mut Arc<Mutex<States>>) -> BoxResult<&Self> {
            if let Some(cmds) = self.command.clone() {
                cmds.handler().await?;
            }
            if self.debug {
                std::env::set_var("RUST_LOG", "debug");
            }
            if self.update {
                tracing::info!("Updating the application...");
            }

            Ok(self)
        }
    }

    impl Default for Cli {
        fn default() -> Self {
            Self::new()
        }
    }
}
