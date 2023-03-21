/*
    Appellation: cmd <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: These commands describe the possible operations of a node in the network
*/
use crate::NetResult;

use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Command {
    StartListening {
        addr: Multiaddr,
        sender: Sender<NetResult>,
    },
    Dial {
        addr: Multiaddr,
        pid: PeerId,
        sender: Sender<NetResult>,
    },
    GetProviders {
        fname: String,
        sender: Sender<HashSet<PeerId>>,
    },
    StartProviding {
        fname: String,
        sender: Sender<()>,
    },
}

impl Command {
    pub fn dial(addr: Multiaddr, pid: PeerId, sender: Sender<NetResult>) -> Self {
        Self::Dial { addr, pid, sender }
    }
    pub fn listen(addr: Multiaddr, sender: Sender<NetResult>) -> Self {
        Self::StartListening { addr, sender }
    }
    pub fn get_provider(fname: String, sender: Sender<HashSet<PeerId>>) -> Self {
        Self::GetProviders { fname, sender }
    }
    pub fn start_providing(fname: String, sender: Sender<()>) -> Self {
        Self::StartProviding { fname, sender }
    }
}
