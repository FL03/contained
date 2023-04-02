/*
    Appellation: setup <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::{Args, ArgAction};
use serde::{Deserialize, Serialize};

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Setup {
    /// Choose a platform to setup
    #[clap(long, short)]
    pub platform: String,
    /// Setup the workspace for WebAssembly workflows
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub wasm: bool
}

impl Setup {
    pub fn new(platform: String, wasm: bool) -> Self {
        Self {
            platform,
            wasm
        }
    }
    
    
}