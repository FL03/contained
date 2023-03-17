/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::rt::{frame::Frame, Runtime};
use crate::{
    clients::Client,
    events::NetworkEvent,
    mainnet::Mainnet,
    peers::{Peer, Peerable},
    NetResult,
};
use libp2p::{futures::StreamExt, Swarm};
use tokio::sync::mpsc;

pub struct Node {
    pub client: Client,
    swarm: Swarm<Mainnet>,
    rt: Runtime,
}

impl Node {
    pub fn new(client: Client, rt: Runtime, swarm: Swarm<Mainnet>) -> Self {
        Self { client, rt, swarm }
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
        let (etx, _) = mpsc::channel::<NetworkEvent>(1);
        let runtime = Runtime::new(arx, etx, Default::default());
        Self::new(
            Client::from(atx),
            runtime,
            peer.swarm(Mainnet::from(peer.pid())),
        )
    }
}
