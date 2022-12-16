/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, settings::*};

pub(crate) mod context;
pub(crate) mod settings;

pub mod cli;
pub mod states;

use scsys::prelude::{BoxResult, Message};
use serde_json::json;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> BoxResult {
    let mut app = Application::default();
    app.start().await?;

    Ok(())
}

pub trait AppSpec: Default {
    type Cnf;
    type Ctx;
    type State;
    fn init() -> Self;
    fn context(&self) -> Self::Ctx;
    fn name(&self) -> String;
    fn settings(&self) -> Self::Cnf;
    fn setup(&mut self) -> BoxResult<&Self>;
    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
    fn state(&self) -> &Arc<Mutex<states::States>>;
}

#[derive(Clone, Debug)]
pub struct Application {
    pub cnf: Settings,
    pub ctx: Context,
    pub state: Arc<Mutex<states::States>>,
}

impl Application {
    pub fn new(cnf: Settings, ctx: Context, state: Arc<Mutex<states::States>>) -> Self {
        cnf.logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::info!("Application initialized; completing setup...");
        Self { cnf, ctx, state }
    }
    /// Initialize the command line interface
    pub async fn cli(&mut self) -> BoxResult<&Self> {
        cli::Cli::default().handle(Arc::clone(self.state())).await?;
        
        Ok(self)
    }
    /// Change the application state
    pub fn set_state(&mut self, state: states::States) -> BoxResult<&Self> {
        self.state = Arc::new(Mutex::new(state.clone()));
        tracing::info!("Update: Application State updated to {}", state);
        Ok(self)
    }
    /// Application runtime
    pub async fn runtime(&mut self) -> BoxResult {
        self.set_state(states::States::Process(Message::from(
            json!({"startup": "success"}),
        )))?;
        // Fetch the initialized cli and process the results
        self.cli().await?;
        Ok(())
    }
    /// Function wrapper for returning the current application state
    pub fn state(&self) -> &Arc<Mutex<states::States>> {
        &self.state
    }
    /// AIO method for running the initialized application
    pub async fn start(&mut self) -> BoxResult<&Self> {
        tracing::info!("Startup: Application initializing...");
        self.runtime().await?;

        Ok(self)
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::from(Context::default())
    }
}

impl std::convert::From<Settings> for Application {
    fn from(data: Settings) -> Self {
        Self::new(data.clone(), Context::from(data), Default::default())
    }
}

impl std::convert::From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(data.clone().cnf, data, Default::default())
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}
