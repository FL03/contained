/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, settings::*, states::*};

pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

pub mod api;
pub mod cli;

use acme::prelude::{AppSpec, AsyncSpawnable, TokioChannelPackMPSC};
use scsys::prelude::{AsyncResult, Locked, State};
use std::{
    convert::From,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() -> AsyncResult {
    // Create an application instance
    let mut app = Application::default();
    // Quickstart the application runtime with the following command
    app.setup()?;
    app.spawn().await?;

    Ok(())
}

#[derive(Debug)]
pub struct Channels {
    pub state: TokioChannelPackMPSC<State<States>>,
}

impl Channels {
    pub fn new(state: TokioChannelPackMPSC<State<States>>) -> Self {
        Self { state }
    }
    pub fn state_channels(&self) -> &TokioChannelPackMPSC<State<States>> {
        &self.state
    }
}

impl Default for Channels {
    fn default() -> Self {
        Self::new(tokio::sync::mpsc::channel(100))
    }
}

#[derive(Debug)]
pub struct Application {
    pub channels: Channels,
    pub ctx: Context,
    pub state: Locked<State<States>>,
}

impl Application {
    pub fn new(channels: Channels, ctx: Context, state: Locked<State<States>>) -> Self {
        Self {
            channels,
            ctx,
            state,
        }
    }
    /// initializes a pack of channels
    pub fn channels<T>(&self, buffer: usize) -> TokioChannelPackMPSC<T> {
        tokio::sync::mpsc::channel::<T>(buffer)
    }
    /// Change the application state
    pub async fn set_state(&mut self, state: States) -> AsyncResult<&Self> {
        // Update the application state
        self.state = Arc::new(Mutex::new(State::new(None, None, Some(state.clone()))));
        // Post the change of state to the according channel(s)
        self.channels(1)
            .0
            .send(self.state.clone())
            .await?;
        tracing::info!("Updating the application state to {}", state);
        Ok(self)
    }
    /// Application runtime
    pub async fn runtime(&mut self) -> AsyncResult {
        // Post the application context to its channels
        // let (sendr, _) = crate::context_channel();
        // let ctx = Arc::new(self.ctx.clone());
        // tokio::spawn(async move {
        //     sendr.send(ctx).await.expect("");
        // }).await?;
        

        let cli = cli::new();
        self.set_state(States::Process).await?;
        // Fetch the initialized cli and process the results
        cli.handler(self.ctx.clone()).await?;
        self.set_state(States::Complete).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncSpawnable for Application {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        tracing::debug!("Spawning the application and related services...");
        self.runtime().await?;
        Ok(self)
    }
}

impl AppSpec<Settings> for Application {
    type Ctx = Context;

    type State = State<States>;

    fn init() -> Self {
        Self::default()
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.clone()
    }

    fn name(&self) -> String {
        self.settings().name.clone()
    }

    fn settings(&self) -> Settings {
        self.ctx.settings().clone()
    }

    fn setup(&mut self) -> AsyncResult<&Self> {
        self.settings().logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::debug!("Application initialized; completing setup...");
        Ok(self)
    }

    fn state(&self) -> &Locked<State<States>> {
        &self.state
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::from(Context::default())
    }
}

impl From<Settings> for Application {
    fn from(data: Settings) -> Self {
        Self::from(Context::from(data))
    }
}

impl From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(Default::default(), data, Default::default())
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}
