/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{cli::Cli, states::appstate::Appstate, Settings};
use scsys::prelude::{BoxResult, Context};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct Application {
    pub cnf: Settings,
    pub ctx: Context<Settings>,
    pub state: Arc<Mutex<Appstate>>,
}

impl Application {
    pub fn new(cnf: Settings, ctx: Context<Settings>, state: Arc<Mutex<Appstate>>) -> Self {
        Self { cnf, ctx, state }
    }
    /// Initialize the command line interface
    pub async fn cli(&mut self) -> BoxResult<Cli> {
        let cli = Cli::default();
        if cli.debug > 0 {
            println!("Debug");
        }
        if cli.command.is_some() {
            match cli.clone().command.unwrap() {
                crate::cli::Commands::Connect { address } => {
                    println!("{:?}", address);
                }
                crate::cli::Commands::System { on } => {
                    if on > 0 {
                        println!("Message Recieved: Initializing the platform...");
                    }
                }
            }
        }

        Ok(cli)
    }
    /// Change the application state
    pub fn set_state(&mut self, state: Appstate) -> BoxResult<&Self> {
        self.state = Arc::new(Mutex::new(state.clone()));
        tracing::info!("Update: Application State updated to {}", state);
        Ok(self)
    }

    /// Application runtime
    pub async fn runtime(&mut self) -> BoxResult {
        self.set_state(Appstate::Process(scsys::prelude::Message::from(
            serde_json::json!({"result": "success"}),
        )))?;
        // Fetch the initialized cli and process the results
        let cli = self.cli().await?;
        tracing::debug!("{:?}", cli);

        Ok(())
    }
    /// Function wrapper for returning the current application state
    pub fn state(&self) -> &Arc<Mutex<Appstate>> {
        &self.state
    }
    /// AIO method for running the initialized application
    pub async fn quickstart(&mut self) -> BoxResult<&Self> {
        self.with_logging();
        tracing::info!("Startup: Application initializing...");
        self.runtime().await?;

        Ok(self)
    }
    /// Initialize application logging
    pub fn with_logging(&mut self) -> &Self {
        self.cnf.logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::debug!("Success: Initialized the logging protocols");
        self
    }
}

impl std::convert::From<Settings> for Application {
    fn from(data: Settings) -> Self {
        Self::new(data.clone(), Context::new(data), Default::default())
    }
}

impl std::convert::From<Context<Settings>> for Application {
    fn from(data: Context<Settings>) -> Self {
        Self::new(data.clone().settings, data, Default::default())
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}
