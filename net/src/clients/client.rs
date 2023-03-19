/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::nodes::rt::cmd::Command;
use crate::NetResult;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Client {
    sender: mpsc::Sender<Command>,
}

impl Client {
    pub fn sender(self) -> mpsc::Sender<Command> {
        self.sender
    }
    /// Listen for incoming connections on the given address.
    pub async fn start_listening(&mut self, addr: Multiaddr) -> NetResult {
        let (tx, rx) = oneshot::channel();
        self.sender.send(Command::listen(addr, tx)).await?;
        rx.await?
    }
    /// Dial the given peer at the given address.
    pub async fn dial(&mut self, pid: PeerId, addr: Multiaddr) -> NetResult {
        let (tx, rx) = oneshot::channel();
        self.sender.send(Command::dial(addr, pid, tx)).await?;
        rx.await?
    }

    /// Advertise the local node as the provider of the given file on the DHT.
    pub async fn start_providing(&mut self, fname: String) {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(Command::start_providing(fname, tx))
            .await
            .expect("Command receiver not to be dropped.");
        rx.await.expect("Sender not to be dropped.");
    }

    /// Find the providers for the given file on the DHT.
    pub async fn get_providers(&mut self, fname: String) -> HashSet<PeerId> {
        let (tx, rx) = oneshot::channel();
        self.sender
            .send(Command::get_provider(fname, tx))
            .await
            .expect("Command receiver not to be dropped.");
        rx.await.expect("Sender not to be dropped.")
    }
}

impl From<mpsc::Sender<Command>> for Client {
    fn from(sender: mpsc::Sender<Command>) -> Client {
        Client { sender }
    }
}

impl Default for Client {
    fn default() -> Self {
        let (tx, _) = mpsc::channel::<Command>(1);
        Self::from(tx)
    }
}
