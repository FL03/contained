/*
    Appellation: dial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::mainnet::Mainnet;
use crate::NetResult;
use libp2p::{Multiaddr, Swarm};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct Listen {
    addr: Multiaddr,
    sender: Sender<NetResult<()>>,
}

impl Listen {
    pub fn new(addr: Multiaddr, sender: Sender<NetResult>) -> Self {
        Self { addr, sender }
    }
    pub fn address(&self) -> &Multiaddr {
        &self.addr
    }
    pub async fn start_listening(self, swarm: &mut Swarm<Mainnet>) {
        let _ = match swarm.listen_on(self.addr) {
            Ok(_) => self.sender.send(Ok(())),
            Err(e) => self.sender.send(Err(Box::new(e))),
        };
    }
    pub fn sender(self) -> Sender<NetResult> {
        self.sender
    }
}
