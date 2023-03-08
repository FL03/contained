/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    rt::{ops::Frame, Runtime},
    Client,
};
use crate::{
    backend::cli,
    events::Event,
    mainnet::Mainnet,
    peers::{Peer, Peerable},
    NetResult,
};
use libp2p::Swarm;
use tokio::sync::mpsc;

pub struct Node {
    client: Client,
    event: mpsc::Receiver<Event>,
    runtime: Runtime,
}

impl Node {
    pub fn new(client: Client, event: mpsc::Receiver<Event>, runtime: Runtime) -> Self {
        Self {
            client,
            event,
            runtime,
        }
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub fn event(self) -> mpsc::Receiver<Event> {
        self.event
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
            tokio::select! {
                Some(event) = self.event.recv() => {
                    println!("{event:?}");
                },
            }
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::from(Peer::default())
    }
}

impl<P: Peerable> From<P> for Node {
    fn from(peer: P) -> Node {
        let (atx, arx) = mpsc::channel::<Frame>(1);
        let (etx, erx) = mpsc::channel::<Event>(1);
        let runtime = Runtime::new(arx, etx, peer.clone().swarm(Mainnet::from(peer.pid())));
        Self::new(atx.into(), erx, runtime)
    }
}
