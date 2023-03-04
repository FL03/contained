/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::actions::{Action, Dial, GetProviders, Listen, StartProviding};
use crate::NetResult;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Client {
    sender: mpsc::Sender<Action>,
}

impl Client {
    pub fn new(sender: mpsc::Sender<Action>) -> Self {
        Self { sender }
    }
    pub fn sender(&self) -> &mpsc::Sender<Action> {
        &self.sender
    }
    /// Listen for incoming connections on the given address.
    pub async fn start_listening(&mut self, addr: Multiaddr) -> NetResult {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Listen::new(addr, sender).into())
            .await
            .expect("Command receiver not to be dropped.");
        receiver.await.expect("Sender not to be dropped.")
    }
    /// Dial the given peer at the given address.
    pub async fn dial(&mut self, peer_id: PeerId, peer_addr: Multiaddr) -> NetResult {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Dial::new(peer_addr, peer_id, sender).into())
            .await
            .expect("Command receiver not to be dropped.");
        receiver.await.expect("Sender not to be dropped.")
    }

    /// Advertise the local node as the provider of the given file on the DHT.
    pub async fn start_providing(&mut self, file_name: String) {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(StartProviding::new(file_name, sender).into())
            .await
            .expect("Command receiver not to be dropped.");
        receiver.await.expect("Sender not to be dropped.");
    }

    /// Find the providers for the given file on the DHT.
    pub async fn get_providers(&mut self, file_name: String) -> HashSet<PeerId> {
        let chan = oneshot::channel();
        self.sender
            .send(GetProviders::new(file_name, chan.0).into())
            .await
            .expect("Command receiver not to be dropped.");
        chan.1.await.expect("Sender not to be dropped.")
    }
}
