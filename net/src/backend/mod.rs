/*
    Appellation: backend <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::context::*;

pub(crate) mod context;

pub mod cli;
pub mod rpc;

use std::sync::Arc;

pub struct Backend {
    cli: Arc<cli::CommandLineInterface>,
}

impl Backend {
    pub async fn handle_cli(&mut self) {}
}
