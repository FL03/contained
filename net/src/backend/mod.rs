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

use crate::nodes::Node;
use crate::NetResult;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct Backend {
    cli: Arc<cli::CommandLineInterface>,
    ctx: Context,
}

impl Backend {
    pub fn new(cli: Arc<cli::CommandLineInterface>, ctx: Context) -> Self {
        Self { cli, ctx }
    }
    pub async fn handle_cli(&mut self) {
        let cli = self.cli.as_ref().clone();
        self.ctx.peer = cli.handle_seed();
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
        tokio::spawn(node.start());
        loop {}
    }
}
