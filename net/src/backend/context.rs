/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    rt::{frame::Frame, Runtime},
    Client,
};
use crate::events::Event;
use crate::peer::{Peer, Peerable};
use crate::{mainnet::Mainnet, NetResult};

use tokio::sync::mpsc;

pub struct Context {
    client: Client,
    event: mpsc::Receiver<Event>,
    peer: Peer,
    runtime: Runtime,
}

impl Context {
    pub fn client(self) -> Client {
        self.client
    }
    pub fn event(self) -> mpsc::Receiver<Event> {
        self.event
    }
    pub fn peer(self) -> Peer {
        self.peer
    }
    pub fn runtime(self) -> Runtime {
        self.runtime
    }
    pub async fn run(self) -> NetResult {
        self.runtime.spawn().await?;

        Ok(())
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
