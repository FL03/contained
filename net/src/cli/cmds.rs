/*
    Appellation: cmds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    Provide {
        #[clap(long, short)]
        name: String,
        #[clap(long, short)]
        path: PathBuf,
    },
    Get {
        #[clap(long, short)]
        name: String,
    },
}
