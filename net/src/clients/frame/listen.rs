/*
    Appellation: dial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::NetResult;
use libp2p::Multiaddr;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Listen {
    addr: Multiaddr,
    sender: oneshot::Sender<NetResult<()>>,
}

impl Listen {
    pub fn new(addr: Multiaddr, sender: oneshot::Sender<NetResult>) -> Self {
        Self { addr, sender }
    }
    pub fn address(&self) -> &Multiaddr {
        &self.addr
    }
    pub fn sender(self) -> oneshot::Sender<NetResult> {
        self.sender
    }
}
