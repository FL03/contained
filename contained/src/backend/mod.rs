pub use self::{context::*, settings::*};

pub(crate) mod context;
pub(crate) mod settings;

pub mod cli;

use crate::clients::Client;
use crate::prelude::Resultant;
use crate::rt::Runtime;
use cli::{Cli, Opts};

pub struct Backend {
    client: Client,
    ctx: Context,
}

impl Backend {
    pub fn new() -> Self {
        let cnf = Settings::default();
        let ctx = Context::new(cnf);

        Self {
            client: Client::default(),
            ctx,
        }
    }
    pub fn context(&self) -> &Context {
        &self.ctx
    }
    pub async fn handle_cli(&mut self, cli: Cli) -> Resultant {
        if let Some(opts) = cli.opts {
            match opts {
                Opts::Execute { space, workload } => {
                    self.client
                        .run_workload(space.unwrap_or_else(|| "origin".to_string()), workload)
                        .await?;
                }
                Opts::Setup { .. } => {}
                Opts::Start { .. } => {}
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
        Runtime::default().spawn();
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
        Self {
            client: Client::default(),
            ctx,
        }
    }
}

impl From<Settings> for Backend {
    fn from(cnf: Settings) -> Self {
        Self::from(Context::new(cnf))
    }
}
