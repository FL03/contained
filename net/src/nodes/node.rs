/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    rt::{frame::Frame, Runtime},
    Client,
};
use crate::{
    events::ClientEvent,
    mainnet::Mainnet,
    peers::{Peer, Peerable},
    NetResult,
};
use libp2p::{futures::StreamExt, multiaddr::Protocol, Swarm};
use std::collections::hash_map;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct Node {
    pub client: Client,
    event: mpsc::Receiver<ClientEvent>,
    swarm: Swarm<Mainnet>,
    rt: Runtime,
}

impl Node {
    pub fn new(
        client: Client,
        event: mpsc::Receiver<ClientEvent>,
        rt: Runtime,
        swarm: Swarm<Mainnet>,
    ) -> Self {
        Self {
            client,
            event,
            rt,
            swarm,
        }
    }
    pub fn event(self) -> mpsc::Receiver<ClientEvent> {
        self.event
    }
    pub fn runtime(&self) -> &Runtime {
        &self.rt
    }
    pub async fn run(mut self) -> NetResult {
        loop {
            tokio::select! {
                Some(act) = self.rt.action.recv() => {
                    tracing::info!("Processing the command");
                    self.rt.exec.handle_command(act, &mut self.swarm).await;
                },
                Some(event) = self.swarm.next() => {
                    tracing::info!("SwarmEvent: processing the network event");
                    self.rt.exec.handle_event(event, &mut self.swarm).await;
                },

            }
        }
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<NetResult> {
        tokio::spawn(self.run())
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
        let (etx, erx) = mpsc::channel::<ClientEvent>(1);
        let runtime = Runtime::new(arx, etx, Default::default());
        Self::new(
            Client::from(atx),
            erx,
            runtime,
            peer.clone().swarm(Mainnet::from(peer.pid())),
        )
    }
}
