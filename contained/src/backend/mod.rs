/*
    Appellation: backend <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, settings::*};

mod context;
mod settings;

pub mod cli;
pub mod rpc;

use crate::net::subnet::{
    node::{Channels, Node},
    Client,
};
use crate::prelude::{peers::*, Resultant};
use cli::{Cli, Opts};

pub struct Backend {
    ctx: Context,
}

impl Backend {
    pub fn new() -> Self {
        let cnf = Settings::default();
        let ctx = Context::new(cnf);

        Self { ctx }
    }
    pub fn context(&self) -> &Context {
        &self.ctx
    }
    pub async fn handle_cli(&mut self, cli: Cli, client: &mut Client, node: Node) -> Resultant {
        if let Some(opts) = cli.opts {
            match opts {
                Opts::Execute { .. } => todo!("Execute command"),
                Opts::Network(net) => {
                    self.ctx.cnf.cluster.seed = net.seed;
                    let peer = self.ctx.peer();
                    tracing::info!("Peer: {:?}", peer.pid());
                    if let Some(addr) = net.dial {
                        let addr = addr.parse().unwrap();
                        tracing::info!("Dialing {} at {}", peer.pid(), addr);
                        client.dial(peer.pid(), addr).await.expect("");
                    }
                    if let Some(addr) = net.listen {
                        let addr = addr.parse().unwrap();
                        tracing::info!("Listening on: {}", addr);
                        client.listen(addr).await.expect("");
                    }
                    if net.up {
                        tracing::info!("Starting network...");
                        if net.detached {
                            tracing::info!("Spawning a detached instance of the node...");
                            let _ = node.spawn();
                        } else {
                            let _ = node.spawn().await.expect("");
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
        let (chan, tx_cmd, mut rx_evt) = Channels::with_capacity(9);
        let node = Node::from((chan, self.ctx.peer()));
        let mut client = Client::new(tx_cmd);
        self.handle_cli(cli, &mut client, node).await?;
        Ok(loop {
            tokio::select! {
                Some(event) = rx_evt.recv() => {
                    tracing::info!("Received event: {:?}", event);
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::warn!("Signal received, terminating the system...");
                    break;
                }
            }
        })
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
