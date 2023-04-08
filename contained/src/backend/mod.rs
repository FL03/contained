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
    Client, NetworkOperator
};
use crate::prelude::{peers::*, Resultant};
use cli::{args::NetworkOpts, Cli, Opts};

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
    pub async fn handle_cli(&mut self, cli: Cli, client: &mut Client, mut node: Node) -> Resultant {
        if let Some(opts) = cli.opts {
            match opts {
                Opts::Agent(_args) => todo!("Execute command"),
                Opts::Network(args) => {
                    // Fix the peer seed setup
                    self.ctx.cnf.cluster.seed = args.seed;
                    let addr = if let Some(addr) = args.addr {
                        addr 
                    } else {
                        crate::prelude::DEFAULT_MULTIADDR.parse().unwrap()
                    };
                    tracing::info!("Listening on: {:?}", node.listen_on(addr));
                    let peer = self.ctx.peer();
                    tracing::info!("Peer: {:?}", peer.pid());
                    if let Some(cmd) = args.cmd {
                        match cmd {
                            NetworkOpts::Dial { addr, pid } => {
                                tracing::info!("Dialing: {:?}", &addr);
                                client.dial(pid, addr).await.expect("");
                            }
                            NetworkOpts::Provide { .. } => todo!("Provide command"),
                            NetworkOpts::Providers { .. } => todo!("Get providers command"),
                        }
                    }
                    if args.up {
                        tracing::info!("Starting network...");
                        if args.detached {
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
        let (chan, mut client, mut event_rx) = Channels::with_capacity(9);
        let node = Node::from((chan, self.ctx.peer()));
        tracing::info!("Peer: {:?}", node.pid());
        self.handle_cli(cli, &mut client, node).await?;
        Ok(loop {
            tokio::select! {
                Some(event) = event_rx.recv() => {
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
