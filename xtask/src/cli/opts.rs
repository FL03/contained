/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::args::{Build, Setup};
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Subcommand)]
pub enum Opts {
    Auto,
    Build(Build),
    Setup(Setup),
    Test {
        #[clap(long, short)]
        package: Option<String>
    }
}

