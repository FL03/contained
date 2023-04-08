/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, primitives::*, specs::*, utils::*};

mod errors;
mod primitives;
mod specs;
mod utils;

pub mod events;
pub mod peers;
pub mod subnet;

use peers::*;

use serde::{Deserialize, Serialize};

pub enum Overlay {
    Mainnet,
    Subnet,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NetworkConfig {
    pub addr: Multiaddr,
    pub seed: Option<u8>,
}

impl NetworkConfig {
    pub fn new(addr: Multiaddr, seed: Option<u8>) -> Self {
        Self { addr, seed }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            addr: DEFAULT_MULTIADDR.parse().unwrap(),
            seed: None,
        }
    }
}

pub struct Starter {
    pub cnf: NetworkConfig,
    pub overlay: Overlay,
}

impl Starter {
    pub fn new(cnf: NetworkConfig, overlay: Overlay) -> Self {
        Self { cnf, overlay }
    }

    pub fn start(
        self,
    ) -> (
        subnet::node::Node,
        subnet::Client,
        tokio::sync::mpsc::Receiver<events::NetworkEvent>,
    ) {
        let peer = if let Some(seed) = self.cnf.seed {
            Peer::try_from(seed).unwrap_or_default()
        } else {
            Peer::default()
        };
        let (chan, client, events) = match self.overlay {
            Overlay::Mainnet => todo!("Mainnet overlay"),
            Overlay::Subnet => subnet::node::Channels::with_capacity(9),
        };
        let mut swarm = peer.swarm();
        swarm.listen_on(self.cnf.addr).unwrap();
        let node = subnet::node::Node::new(chan, swarm);
        (node, client, events)
    }
}
