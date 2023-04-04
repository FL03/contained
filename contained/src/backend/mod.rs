pub use self::{context::*, settings::*};

pub(crate) mod context;
pub(crate) mod settings;

pub mod cli;

use crate::net::peers::*;
use crate::net::subnet::node::Node;
use crate::prelude::Resultant;
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
    pub async fn handle_cli(&mut self, cli: Cli) -> Resultant {
        if let Some(opts) = cli.opts {
            match opts {
                Opts::Execute { .. } => todo!("Execute command"),
                Opts::Network(net) => {
                    let peer = if let Some(seed) = net.seed {
                        Peer::try_from(seed).unwrap_or_default()
                    } else {
                        Peer::default()
                    };
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

impl From<Context> for Backend {
    fn from(ctx: Context) -> Self {
        Self { ctx }
    }
}

impl From<Settings> for Backend {
    fn from(cnf: Settings) -> Self {
        Self::from(Context::new(cnf))
    }
}
