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

use crate::{nodes::Node, NetResult};
use cli::{Command, CommandLineInterface};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct Backend {
    cli: Arc<cli::CommandLineInterface>,
    ctx: Context,
}

impl Backend {
    pub fn new(cli: Arc<CommandLineInterface>, ctx: Context) -> Self {
        Self { cli, ctx }
    }
    pub async fn handle_cli(&mut self) {
        let cli = self.cli.as_ref().clone();
        self.ctx.peer = cli.handle_seed();
        if let Some(opts) = cli.clone().cmd() {
            match opts {
                Command::Get { .. } => {}
                Command::Provide(provide) => {}
            }
        }
    }
    pub fn node(self) -> Node {
        Node::from(self.ctx.peer())
    }
    pub async fn run(&mut self) -> NetResult {
        tracing_subscriber::fmt::init();
        tracing::info!("Processing inputs...");
        self.handle_cli().await;
        tracing::info!("Initializing the network...");
        let node = self.clone().node();
        node.spawn();
        loop {}
    }
}
