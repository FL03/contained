/*
    Appellation: dial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::NetResult;
use libp2p::{Multiaddr, PeerId};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct Dial {
    addr: Multiaddr,
    pid: PeerId,
    sender: Sender<NetResult<()>>,
}

impl Dial {
    pub fn new(addr: Multiaddr, pid: PeerId, sender: Sender<NetResult>) -> Self {
        Self { addr, pid, sender }
    }
    pub fn address(&self) -> &Multiaddr {
        &self.addr
    }
    pub fn pid(&self) -> &PeerId {
        &self.pid
    }
    pub fn sender(self) -> Sender<NetResult> {
        self.sender
    }
}