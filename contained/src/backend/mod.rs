pub use self::{context::*, settings::*};

mod context;
mod settings;

pub mod cli;

use crate::net::peers::*;
use crate::net::subnet::{
    client::Client,
    node::{Channels, NetworkEventRx, Node},
};
use crate::prelude::Resultant;
use cli::{Cli, Opts};

pub struct Backend {
    client: Client,
    ctx: Context,
}

impl Backend {
    pub fn new() -> Self {
        let cnf = Settings::default();
        let ctx = Context::new(cnf);

        let (client, _) = Client::with_capacity(9);
        Self { client, ctx }
    }
    pub fn context(&self) -> &Context {
        &self.ctx
    }
    pub async fn handle_cli(&mut self, cli: Cli) -> Resultant {
        if let Some(opts) = cli.opts {
            match opts {
                Opts::Execute { .. } => todo!("Execute command"),
                Opts::Network(net) => {
                    self.ctx.cnf.cluster.seed = net.seed;
                    let peer = self.ctx.peer();
                    tracing::info!("Peer: {:?}", peer.pid());

                    let network = Node::from(peer);
                    if net.up {
                        tracing::info!("Starting network...");
                        if net.detached {
                            tracing::info!("Spawning a detached instance of the node...");
                            let _ = network.spawn();
                        } else {
                            let _ = network.spawn().await.expect("");
                        }
                    }
                }
                Opts::Setup { .. } => todo!("Setup command"),
            }
        };

        Ok(())
    }
    pub async fn run(mut self) -> Resultant {
        let cli = Cli::default();
        self.handle_cli(cli).await?;
        Ok(())
    }
    pub fn settings(&self) -> &Settings {
        self.ctx.settings()
    }
    pub fn setup(self) -> Self {
        // Initialize tracing layer...
        let logger = self.ctx.settings().logger.clone();
        logger.setup_env(None).init_tracing();
        self
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<Resultant> {
        tokio::spawn(self.run())
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}
