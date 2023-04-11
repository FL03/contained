/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::opts::*;

pub(crate) mod opts;

pub mod args;

use clap::{ArgAction, Parser};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct CommandLineInterface {
    /// Network specific commands
    #[clap(subcommand)]
    pub cmd: Option<Opts>,
    /// Optionally execute the program in production mode
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub release: bool,
    /// Startup 
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub up: bool,
    /// Signal for more in-depth logging
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub verbose: bool,
    /// Build all 
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub workspace: bool,
}

impl CommandLineInterface {
    pub fn cmd(&self) -> &Option<Opts> {
        &self.cmd
    }
    pub fn release(&self) -> bool {
        self.release
    }
    pub fn up(&self) -> bool {
        self.up
    }
    pub fn verbose(&self) -> bool {
        self.verbose
    }
    pub fn workspace(&self) -> bool {
        self.workspace
    }
}
impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::parse()
    }
}