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
    use clap::Parser;
    use serde::{Deserialize, Serialize};

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
    }

    impl Default for Cli {
        fn default() -> Self {
            Self::new()
        }
    }
}
