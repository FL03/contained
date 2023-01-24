/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::cli::{cmd::Commands, CommandLineInterface};
use crate::Context;
use acme::prelude::{AsyncSpawnable, Session};
use clap::Parser;
use scsys::prelude::AsyncResult;
use std::sync::Arc;
use tokio::sync::{oneshot, watch};
use tokio::task::JoinHandle;

pub async fn handle_cli(cli: CommandLineInterface) -> AsyncResult {
    if let Some(cmd) = cli.command() {
        match cmd {
            Commands::System(sys) => {
                if sys.up {
                    tracing::info!("Message Recieved: System initializing...");
                    
                }
            }
        }
    }

    Ok(())
}

pub enum Surfaces {
    API = 0,
    CLI = 1,
    RPC = 2,
    P2P = 3,
    WebRTC = 4,
}

pub struct Interface<T>(T, Surfaces);

#[derive(Clone, Debug)]
pub struct Runtime {
    pub ctx: watch::Receiver<Arc<Context>>,
    pub cli: Arc<CommandLineInterface>,
    pub session: Session,
}

impl Runtime {
    pub fn new(ctx: Arc<Context>, session: Session) -> Self {
        Self {
            cli: CommandLineInterface::parse().into(),
            ctx: watch::channel(ctx.clone()).1,
            session,
        }
    }
    pub async fn handle(&self) -> JoinHandle<AsyncResult> {
        let rt = Arc::new(self.clone());
        let ctx = watch::channel(self.ctx.clone());

        tokio::spawn(async move {
            let cli = rt.cli.clone();
            loop {
                // Watch for updates to the application context
                if rt.ctx.has_changed().expect("Context channel droppped...") {
                    let ctx = rt.ctx.borrow().clone();
                    tracing::info!("Context changed: {:?}", ctx);
                }

                handle_cli(cli.as_ref().clone()).await?;
            }
        })
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        handle_cli(self.cli.as_ref().clone()).await?;

        Ok(self)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::from(Context::default())
    }
}

impl From<Arc<Context>> for Runtime {
    fn from(ctx: Arc<Context>) -> Self {
        Self::new(ctx, Default::default())
    }
}

impl From<Context> for Runtime {
    fn from(ctx: Context) -> Self {
        Self::from(Arc::new(ctx))
    }
}
