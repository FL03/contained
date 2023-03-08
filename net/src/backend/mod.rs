/*
    Appellation: backend <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{context::*, settings::*};

pub(crate) mod context;
pub(crate) mod settings;

pub mod cli;
pub mod rpc;

use crate::NetResult;
use std::sync::Arc;

pub struct Backend {
    cli: Arc<cli::CommandLineInterface>,
    ctx: Context,
}

impl Backend {
    pub async fn handle_cli(&mut self) {
        let cli = self.cli.as_ref().clone();
        self.ctx.peer = cli.handle_seed();

    }
    pub async fn run(mut self) -> NetResult {
        loop {

        }
    }
}
