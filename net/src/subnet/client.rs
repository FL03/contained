/*
    Appellation: clients <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::layer::Command;
use crate::NetworkResult;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};

pub trait NetworkClient {
    type Command;

    fn command(self) -> mpsc::Sender<Self::Command>;
}

pub struct Client {
    pub cmd: mpsc::Sender<Command>,
}

impl Client {
    pub fn new(capacity: usize) -> Self {
        Self {
            cmd: mpsc::channel(capacity).0,
        }
    }
    pub fn sender(self) -> mpsc::Sender<Command> {
        self.cmd
    }
    /// Listen for incoming connections on the given address.
    pub async fn start_listening(&mut self, addr: Multiaddr) -> NetworkResult {
        let (tx, rx) = oneshot::channel();
        self.cmd.send(Command::listen(addr, tx)).await?;
        rx.await?
    }
    /// Dial the given peer at the given address.
    pub async fn dial(&mut self, pid: PeerId, addr: Multiaddr) -> NetworkResult {
        let (tx, rx) = oneshot::channel();
        self.cmd.send(Command::dial(addr, pid, tx)).await?;
        rx.await?
    }

    /// Advertise the local node as the provider of the given file on the DHT.
    pub async fn start_providing(&mut self, fname: String) {
        let (tx, rx) = oneshot::channel();
        self.cmd
            .send(Command::start_providing(fname, tx))
            .await
            .expect("Command receiver not to be dropped.");
        rx.await.expect("Sender not to be dropped.");
    }

    /// Find the providers for the given file on the DHT.
    pub async fn get_providers(&mut self, fname: String) -> HashSet<PeerId> {
        let (tx, rx) = oneshot::channel();
        self.cmd
            .send(Command::get_provider(fname, tx))
            .await
            .expect("Command receiver not to be dropped.");
        rx.await.expect("Sender not to be dropped.")
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(9)
    }
}
