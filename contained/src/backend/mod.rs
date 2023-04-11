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
    NetworkClient,
};
use crate::prelude::{Resultant, Shared};
use cli::{args::NetworkOpts, Cli, Opts};
use tokio::runtime;

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
    pub async fn handle_cli(
        &mut self,
        cli: Cli,
        client: &mut impl NetworkClient,
        mut node: Node,
    ) -> Resultant {
        if let Some(opts) = cli.opts {
            match opts {
                Opts::Agent(_args) => todo!("Execute command"),
                Opts::Network(args) => {
                    if let Some(addr) = args.addr {
                        self.ctx.cnf.network.subnet.addr = addr;
                    };
                    tracing::info!(
                        "Listening on: {:?}",
                        node.listen_on(self.ctx.cnf.network.subnet.addr.clone())
                    );
                    if let Some(cmd) = args.cmd {
                        match cmd {
                            NetworkOpts::Dial { addr, pid } => {
                                tracing::info!("Dialing: {:?}", &addr);
                                client.dial(addr, pid).await?;
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
    pub fn spawn(
        self,
        rt: &runtime::Handle,
    ) -> tokio::task::JoinHandle<Result<Resultant, tokio::task::JoinError>> {
        rt.spawn(tokio::spawn(self.run()))
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}
