/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::frame::Frame;
use crate::NetResult;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Client {
    sender: mpsc::Sender<Frame>,
}

impl Client {
    pub fn new(sender: mpsc::Sender<Frame>) -> Self {
        Self { sender }
    }
    pub fn sender(self) -> mpsc::Sender<Frame> {
        self.sender
    }
    /// Listen for incoming connections on the given address.
    pub async fn start_listening(&mut self, addr: Multiaddr) -> NetResult {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Frame::listen(addr, sender)).await?;
        receiver.await?
    }
    /// Dial the given peer at the given address.
    pub async fn dial(&mut self, pid: PeerId, addr: Multiaddr) -> NetResult {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(Frame::dial(addr, pid, sender)).await?;
        receiver.await?
    }

    /// Advertise the local node as the provider of the given file on the DHT.
    pub async fn start_providing(&mut self, fname: String) {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(Frame::provide(fname, sender))
            .await
            .expect("Command receiver not to be dropped.");
        receiver.await.expect("Sender not to be dropped.");
    }

    /// Find the providers for the given file on the DHT.
    pub async fn get_providers(&mut self, fname: String) -> HashSet<PeerId> {
        let chan = oneshot::channel();
        self.sender
            .send(Frame::get(fname, chan.0))
            .await
            .expect("Command receiver not to be dropped.");
        chan.1.await.expect("Sender not to be dropped.")
    }
}

impl Default for Client {
    fn default() -> Self {
        let (tx, _) = mpsc::channel::<Frame>(1);
        Self::new(tx)
    }
}
