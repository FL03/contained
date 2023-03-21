/*
    Appellation: mainnet <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module is primarily responsible for implementing the described behaviour of the network
*/
pub use self::events::*;

pub(crate) mod events;

use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::swarm::NetworkBehaviour;
use libp2p::{mdns, ping, PeerId};

/// [Conduct] describes the behavior of our network
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
pub struct Mainnet {
    pub freq: ping::Behaviour,
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
}

impl Mainnet {
    pub fn new(kademlia: Kademlia<MemoryStore>, mdns: mdns::tokio::Behaviour) -> Self {
        Self {
            freq: Default::default(),
            kademlia,
            mdns,
        }
    }
}

impl From<PeerId> for Mainnet {
    fn from(pid: PeerId) -> Self {
        let kademlia = Kademlia::new(pid, MemoryStore::new(pid));
        Self::new(
            kademlia,
            mdns::tokio::Behaviour::new(mdns::Config::default(), pid).unwrap(),
        )
    }
}
