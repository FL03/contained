/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::node::rt::frame::Frame;
use crate::NetResult;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Client {
    sender: mpsc::Sender<Frame>,
}

impl Client {
    pub fn sender(self) -> mpsc::Sender<Frame> {
        self.sender
    }
    /// Listen for incoming connections on the given address.
    pub async fn start_listening(&mut self, addr: Multiaddr) -> NetResult {
        let (tx, rx) = oneshot::channel();
        self.sender.send(Frame::listen(addr, tx)).await?;
        rx.await?
    }
    /// Dial the given peer at the given address.
    pub async fn dial(&mut self, pid: PeerId, addr: Multiaddr) -> NetResult {
        let (tx, rx) = oneshot::channel();
        self.sender.send(Frame::dial(addr, pid, tx)).await?;
        rx.await?
    }

    /// Advertise the local node as the provider of the given file on the DHT.
    pub async fn start_providing(&mut self, fname: String) {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(Frame::provide(fname, tx))
            .await
            .expect("Command receiver not to be dropped.");
        rx.await.expect("Sender not to be dropped.");
    }

    /// Find the providers for the given file on the DHT.
    pub async fn get_providers(&mut self, fname: String) -> HashSet<PeerId> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(Frame::get(fname, tx))
            .await
            .expect("Command receiver not to be dropped.");
        rx.await.expect("Sender not to be dropped.")
    }
}

impl From<mpsc::Sender<Frame>> for Client {
    fn from(sender: mpsc::Sender<Frame>) -> Client {
        Client { sender }
    }
}

impl Default for Client {
    fn default() -> Self {
        let (tx, _) = mpsc::channel::<Frame>(1);
        Self::from(tx)
    }
}
