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

use crate::{events::NetworkEvent, nodes::Node, NetResult};
use cli::{CommandLineInterface, Opts};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Backend {
    cli: Arc<cli::CommandLineInterface>,
    ctx: Context,
    event: mpsc::Receiver<NetworkEvent>,
}

impl Backend {
    pub fn new(
        cli: Arc<CommandLineInterface>,
        ctx: Context,
        event: mpsc::Receiver<NetworkEvent>,
    ) -> Self {
        Self { cli, ctx, event }
    }
    pub async fn handle_cli(&mut self, cli: cli::CommandLineInterface) {
        self.ctx.peer = cli.handle_seed();
        if let Some(opts) = cli.cmd() {
            match opts {
                Opts::Get { .. } => {}
                Opts::Provide(_provide) => loop {
                    if let Some(event) = self.event.recv().await {
                        match event {
                            NetworkEvent::InboundRequest { .. } => todo!(),
                        }
                    }
                },
            }
        }
    }
    pub fn node(self) -> Node {
        Node::from(self.ctx.peer())
    }
    pub async fn run(mut self) -> NetResult {
        // Initialize the tracing layer
        tracing_subscriber::fmt::init();
        tracing::info!("Initializing the network...");
        let node = Node::from(self.ctx.peer());
        node.spawn();
        tracing::info!("Processing inputs...");
        self.handle_cli(self.cli.as_ref().clone()).await;

        loop {}
    }
}

impl Default for Backend {
    fn default() -> Self {
        let (_, rx) = mpsc::channel(1);
        Self {
            cli: cli::CommandLineInterface::default().into(),
            ctx: Default::default(),
            event: rx,
        }
    }
}
