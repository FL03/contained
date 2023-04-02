/*
    Appellation: cmd <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: These commands describe the possible operations of a node in the network
*/
use crate::NetworkResult;

use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Command {
    StartListening {
        addr: Multiaddr,
        sender: Sender<NetworkResult>,
    },
    Dial {
        addr: Multiaddr,
        pid: PeerId,
        sender: Sender<NetworkResult>,
    },
    GetProviders {
        id: String,
        sender: Sender<HashSet<PeerId>>,
    },
    StartProviding {
        id: String,
        sender: Sender<()>,
    },
}

impl Command {
    pub fn dial(addr: Multiaddr, pid: PeerId, sender: Sender<NetworkResult>) -> Self {
        Self::Dial { addr, pid, sender }
    }
    pub fn listen(addr: Multiaddr, sender: Sender<NetworkResult>) -> Self {
        Self::StartListening { addr, sender }
    }
    pub fn get_provider(id: String, sender: Sender<HashSet<PeerId>>) -> Self {
        Self::GetProviders { id, sender }
    }
    pub fn start_providing(id: String, sender: Sender<()>) -> Self {
        Self::StartProviding { id, sender }
    }
}
