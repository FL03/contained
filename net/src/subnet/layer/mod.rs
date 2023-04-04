/*
    Appellation: layer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: These commands describe the possible operations of a node in the network
*/
pub use self::{command::*, connection::*};

mod command;
mod connection;

use crate::NetworkResult;
use async_trait::async_trait;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};

#[async_trait]
pub trait SubnetManager: Send + Sync {
    fn sender(&self) -> &mpsc::Sender<Command>;
    /// Listen for incoming connections on the given address.
    async fn start_listening(&mut self, addr: Multiaddr) -> NetworkResult {
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::listen(addr, tx)).await?;
        rx.await?
    }
    /// Dial the given peer at the given address.
    async fn dial(&mut self, pid: PeerId, addr: Multiaddr) -> NetworkResult {
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::dial(addr, pid, tx)).await?;
        rx.await?
    }

    /// Advertise the local node as the provider of the given file on the DHT.
    async fn start_providing(&mut self, fname: String) {
        let (tx, rx) = oneshot::channel();
        self.sender()
            .send(Command::start_providing(fname, tx))
            .await
            .expect("Command receiver not to be dropped.");
        rx.await.expect("Sender not to be dropped.");
    }
    /// Find the providers for the given file on the DHT.
    async fn get_providers(&mut self, fname: String) -> HashSet<PeerId> {
        let (tx, rx) = oneshot::channel();
        self.sender()
            .send(Command::get_provider(fname, tx))
            .await
            .expect("Command receiver not to be dropped.");
        rx.await.expect("Sender not to be dropped.")
    }
}

impl SubnetManager for mpsc::Sender<Command> {
    fn sender(&self) -> &mpsc::Sender<Command> {
        self
    }
}
