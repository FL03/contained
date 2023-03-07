/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    cli,
    rt::{frame::Frame, Runtime},
    Client,
};
use crate::events::Event;
use crate::mainnet::Mainnet;
use crate::peer::{Peer, Peerable};
use crate::NetResult;
use tokio::sync::mpsc;

pub struct Context {
    client: Client,
    event: mpsc::Receiver<Event>,
    peer: Peer,
    runtime: Runtime,
}

impl Context {
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub fn event(self) -> mpsc::Receiver<Event> {
        self.event
    }
    pub fn peer(self) -> Peer {
        self.peer
    }
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }
    pub async fn start(mut self, cli: cli::CommandLineInterface) -> NetResult {
        // Startup the network in the background
        self.runtime.spawn();
        // Process the inputs
        cli.handle(&mut self.client).await?;
        loop {
            // tokio::select! {
            //     Ok(_) = cli.handle(&mut self.client) => {},
            // }
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context::from(Peer::default())
    }
}

impl From<Peer> for Context {
    fn from(peer: Peer) -> Context {
        let (atx, arx) = mpsc::channel::<Frame>(1);
        let (etx, event) = mpsc::channel::<Event>(1);
        let client = atx.into();
        let runtime = Runtime::new(
            arx,
            etx,
            peer.clone().swarm(Mainnet::from(peer.clone().pid())),
        );
        Self {
            client,
            event,
            peer,
            runtime,
        }
    }
}
