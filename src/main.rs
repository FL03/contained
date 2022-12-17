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
use std::{convert::From, sync::{mpsc, Arc, Mutex}, thread::JoinHandle};

#[tokio::main]
async fn main() -> BoxResult {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        tx.send("test").unwrap();

    });
    println!("{:?}", rx.recv()?);
    let mut app = Application::default();
    app.start().await?;

    Ok(())
}


pub struct Conduit<T> {
    pub receiver: mpsc::Receiver<T>,
    pub sender: mpsc::Sender<T>,
}

impl<T> Conduit<T> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { receiver, sender }
    }
}

impl<T> Default for Conduit<T> where T: Default {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Handler<T: Send + Sync + 'static> {
    fn handle(&self) -> JoinHandle<T>;
    fn spawn(&self) -> BoxResult<&Self>;
}

pub trait Contract: Send + Sync + 'static {

}

pub trait Transformation<S> {
    type Error;
    type Res;

    fn transform(&self, data: S) -> Result<Self::Res, Self::Error>;
}

pub trait Spawnable: Send + Sync + 'static {
    type Error;
    fn handle(&self) -> JoinHandle<&Self>;

}

pub fn detached_handle<S: Clone + Send + Sync + 'static, T: Send + Sync + 'static>(data: S, transform: fn(S) -> Arc<T>) -> BoxResult<JoinHandle<Arc<T>>> {
    let handle = std::thread::spawn( move || {
        std::thread::spawn(move || {
            tracing::info!("Spawned the detached thread");

        });
        transform(data.clone())
    });

    Ok(handle)
}
/// A minimal function wrapper for
pub fn spawner<F: Send + Sync + 'static, T>(handle: F) -> JoinHandle<F> {
    std::thread::spawn( move || {
        handle
    })
}

pub fn handler<T: Send + Sync + 'static>(data: T, transform: fn(T) -> Arc<T>) -> BoxResult<JoinHandle<Arc<T>>> {
    let handle = std::thread::spawn( move || {
        transform(data)
    });

    Ok(handle)
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
    pub fn channels<T>(&self) -> Conduit<T> {
        Conduit::new()
    }
    /// Initialize the command line interface
    pub fn cli(&mut self) -> BoxResult<JoinHandle<Arc<cli::Cli>>> {
        let handle = std::thread::Builder::new().name("runtime".to_string()).spawn(move || {
            let cli = Arc::from(cli::new());

            cli.handle();
            Arc::clone(&cli)
        })?;        
        Ok(handle)
    }
    /// Change the application state
    pub fn set_state(&mut self, state: states::States) -> BoxResult<&Self> {
        self.state = Arc::new(Mutex::new(state.clone()));
        self.channels().sender.send(Arc::clone(&self.state)).unwrap();
        tracing::info!("Update: Application State updated to {}", state);
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
