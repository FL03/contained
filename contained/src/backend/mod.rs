pub use self::{context::*, settings::*};

pub(crate) mod context;
pub(crate) mod settings;

pub mod cli;

use crate::prelude::Resultant;
use cli::{Cli, Opts};
use std::sync::Arc;

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
                Opts::Setup { .. } => {
                    self.setup();
                }
                Opts::Start { .. } => {}
            }
        };

        Ok(())
    }
    pub async fn run(mut self) -> Resultant {
        loop {}
    }
    pub fn settings(&self) -> &Settings {
        self.ctx.settings()
    }
    pub fn setup(&self) {
        self.ctx.setup();
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<Resultant> {
        tokio::spawn(self.run())
    }
}

impl Default for Backend {
    fn default() -> Self {
        let ctx = Context::default();
        Self { ctx }
    }
}

impl From<Context> for Backend {
    fn from(ctx: Context) -> Self {
        Self {
            ctx
        }
    }
}

impl From<Settings> for Backend {
    fn from(cnf: Settings) -> Self {
        Self::from(Context::new(cnf))
    }
}
