/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::opts::*;

mod opts;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    pub opts: Option<Opts>,
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}
