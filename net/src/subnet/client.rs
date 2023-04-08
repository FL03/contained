/*
    Appellation: clients <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::layer::Command;
use crate::subnet::proto::reqres::Response;
use crate::NetworkResult;
use async_trait::async_trait;
use libp2p::request_response::ResponseChannel;
use libp2p::{core::transport::ListenerId, Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot};

#[async_trait]
pub trait NetworkClient: Send + Sync {
    fn sender(&self) -> &mpsc::Sender<Command>;
    /// Dial the given peer at the given address.
    async fn dial(&mut self, pid: PeerId, addr: Multiaddr) -> NetworkResult {
        tracing::info!("Dialing {} at {}", pid, addr);
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::dial(addr, pid, tx)).await?;
        rx.await?
    }
    /// Listen for incoming connections on the given address.
    async fn listen(&mut self, addr: Multiaddr) -> NetworkResult<ListenerId> {
        let (tx, rx) = oneshot::channel();
        tracing::info!("Listening for incoming connections on {}", addr);
        self.sender().send(Command::listen(addr, tx)).await?;
        rx.await?
    }
    /// Advertise the local node as the provider of the given file on the DHT.
    async fn provide(&mut self, cid: String) -> NetworkResult {
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::provide(cid, tx)).await?;
        rx.await?
    }
    /// Find the providers for the given file on the DHT.
    async fn providers(&mut self, cid: String) -> NetworkResult<HashSet<PeerId>> {
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::providers(cid, tx)).await?;
        rx.await?
    }
    /// Request the content of the given file from the given peer.
    async fn request(&mut self, payload: String, peer: PeerId) -> NetworkResult<Response> {
        let (tx, rx) = oneshot::channel();
        self.sender()
            .send(Command::Request { payload, peer, tx })
            .await?;
        rx.await?
    }
    /// Respond with the provided file content to the given request.
    async fn respond(&mut self, payload: Vec<u8>, channel: ResponseChannel<Response>) {
        self.sender()
            .send(Command::Respond { payload, channel })
            .await
            .expect("Command receiver not to be dropped.");
    }
}

impl NetworkClient for mpsc::Sender<Command> {
    fn sender(&self) -> &mpsc::Sender<Command> {
        self
    }
}

pub struct Client {
    pub cmd: mpsc::Sender<Command>,
}

impl Client {
    pub fn new(cmd: mpsc::Sender<Command>) -> Self {
        Self { cmd }
    }
    pub fn with_capacity(capacity: usize) -> (Self, mpsc::Receiver<Command>) {
        let (tx, rx) = mpsc::channel(capacity);
        (Self::new(tx), rx)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::with_capacity(9).0
    }
}

impl NetworkClient for Client {
    fn sender(&self) -> &mpsc::Sender<Command> {
        &self.cmd
    }
}
