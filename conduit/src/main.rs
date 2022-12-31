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

use acme::prelude::AppSpec;
use scsys::prelude::{BoxResult, Locked, State};
use std::{
    convert::From,
    sync::{Arc, Mutex},
};

const DEFAULT_STATE_CHANNEL: usize = 999;

#[tokio::main]
async fn main() -> BoxResult {
    // Create an application instance
    let mut app = Application::default();
    // Quickstart the application runtime with the following command
    app.start().await?;

    Ok(())
}

pub type TokioChannelPackMPSC<T = ()> =
    (tokio::sync::mpsc::Sender<T>, tokio::sync::mpsc::Receiver<T>);

pub trait ChannelSpec: std::fmt::Debug {
    type Msg: Clone + Default + ToString;

    fn buffer(&self) -> usize;
    fn channel(&self) -> TokioChannelPackMPSC<Self::Msg> {
        tokio::sync::mpsc::channel(self.buffer())
    }
    fn sender(&self) -> tokio::sync::mpsc::Sender<Self::Msg> {
        self.channel().0
    }
    fn receiver(&self) -> tokio::sync::mpsc::Receiver<Self::Msg> {
        self.channel().1
    }
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
        Self::new(tokio::sync::mpsc::channel(DEFAULT_STATE_CHANNEL))
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
        ctx.cnf.logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::info!("Application initialized; completing setup...");
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
    pub async fn set_state(&mut self, state: State<States>) -> BoxResult<&Self> {
        // Update the application state
        self.state = Arc::new(Mutex::new(state.clone()));
        // Post the change of state to the according channel(s)
        self.channels(1).0.send(self.state.clone()).await?;
        tracing::info!("Updating the application state to {}", state);
        Ok(self)
    }
    /// Application runtime
    pub async fn runtime(&mut self) -> BoxResult {
        let cli = cli::new();
        self.set_state(State::new(None, None, Some(States::Process)))
            .await?;
        // Fetch the initialized cli and process the results
        cli.handler().await?;
        self.set_state(State::new(None, None, Some(States::Complete)))
            .await?;
        Ok(())
    }
    /// AIO method for running the initialized application
    pub async fn start(&mut self) -> BoxResult<&Self> {
        tracing::info!("Startup: Application initializing...");
        self.runtime().await?;

        Ok(self)
    }
}

impl AppSpec for Application {
    type Cnf = Settings;

    type Ctx = Context;

    type State = State<States>;

    fn init() -> Self {
        Self::default()
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.clone()
    }

    fn name(&self) -> String {
        String::from("Conduit")
    }

    fn settings(&self) -> Self::Cnf {
        self.ctx.settings().clone()
    }

    fn setup(&mut self) -> BoxResult<&Self> {
        tracing_subscriber::fmt::init();
        tracing::info!("Application initialized; completing setup...");

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
