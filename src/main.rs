/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, settings::*, utils::*};

pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod utils;

pub mod cli;
pub mod states;

use scsys::prelude::{BoxResult, Message};
use serde_json::json;
use std::{convert::From, sync::{mpsc, Arc, Mutex}};
use tokio::{sync, task};
pub type ChannelPackStd<T> = (mpsc::Sender<T>, mpsc::Receiver<T>);

pub type Locked<T> = Arc<Mutex<T>>;

pub async fn fundamental() -> Message {
    let msg = Message::from(json!({"view": "inner"}));
    msg
}

pub async fn middle(mut rz: tokio::sync::mpsc::Receiver<Message>) -> String {
    let res = rz.recv().await.unwrap().to_string();
    res
}

pub async fn outer(mut ry:  tokio::sync::mpsc::Receiver<String>) -> String {
    let res = ry.recv().await.unwrap();
    res
}

#[tokio::main]
async fn main() -> BoxResult {
    // Create an application instance
    let mut app = Application::default();
    // Quickstart the application runtime with the following command
    app.start().await?;

    
    Ok(())
}

pub async fn sample_handler() -> BoxResult {
    let bufs = [0, 1, 2];
    
    // Initialize the asynchronous sender / receiver for the given channel
    let (tx, mut rx) = sync::mpsc::channel(1);
    let (ty, mut ry) = sync::mpsc::channel(2);
    let (tz, mut rz) = sync::mpsc::channel(3);
    tokio::spawn(async move {
        tokio::spawn(async move {
            tokio::spawn(async move {
                tz.send(fundamental().await).await.expect("");
            });
            let mut msg = rz.recv().await.unwrap();
            msg.push(json!({"view": "middle"}));
            ty.send(middle(rz).await).await.expect("");
        });
        tx.send(outer(ry).await).await.expect("");
    });
    println!("{:?}", rx.recv().await.unwrap());
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
    fn state(&self) -> &Locked<states::States>;
}

#[derive(Debug)]
pub struct ApplicationChannels {
    pub state: sync::mpsc::Sender<Arc<states::States>>
}

#[derive(Clone, Debug)]
pub struct Application {
    pub cnf: Settings,
    pub ctx: Context,
    pub state: Locked<states::States>,
}

impl Application {
    pub fn new(cnf: Settings, ctx: Context, state: Locked<states::States>) -> Self {
        cnf.logger().clone().setup(None);
        tracing_subscriber::fmt::init();
        tracing::info!("Application initialized; completing setup...");
        Self { cnf, ctx, state }
    }
    // initializes a pack of channels
    pub fn channels<T>(&self) -> ChannelPackStd<T> {
        mpsc::channel::<T>()
    }
    /// Initialize the command line interface
    pub fn cli(&mut self) -> BoxResult<task::JoinHandle<Arc<cli::Cli>>> {
        let cli = Arc::new(cli::new());
        let handle = tokio::spawn(async move {
            cli.handle();
            cli
        });   
        Ok(handle)
    }
    /// Change the application state
    pub fn set_state(&mut self, state: states::States) -> BoxResult<&Self> {
        // Update the application state
        self.state = Arc::new(Mutex::new(state.clone()));
        // Post the change of state to the according channel(s)
        self.channels().0.send(self.state.clone()).unwrap();
        tracing::info!("Updating the application state to {}", state);
        Ok(self)
    }
    /// Application runtime
    pub async fn runtime(&mut self) -> BoxResult {
        self.set_state(states::States::Process(Message::from(
            json!({"startup": "success"}),
        )))?;
        // Fetch the initialized cli and process the results
        // self.cli().await?;
        cli::new().handler()?;
        Ok(())
    }
    /// Function wrapper for returning the current application state
    pub fn state(&self) -> &Locked<states::States> {
        &self.state
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

    type State = states::States;

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
        self.cnf.clone()
    }

    fn setup(&mut self) -> BoxResult<&Self> {
        tracing_subscriber::fmt::init();
        tracing::info!("Application initialized; completing setup...");

        Ok(self)
    }

    fn state(&self) -> &Arc<Mutex<states::States>> {
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
        Self::new(data.clone(), Context::from(data), Default::default())
    }
}

impl From<Context> for Application {
    fn from(data: Context) -> Self {
        Self::new(data.clone().cnf, data, Default::default())
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx).unwrap())
    }
}
