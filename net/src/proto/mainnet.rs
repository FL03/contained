/*
    Appellation: mainnet <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        The mainnet describes interactions between subnets; this is accomplished in several ways including the binding of a namespace to the system.
*/
use crate::events::Events;
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::swarm::NetworkBehaviour;
use libp2p::{mdns, ping};

/// [Conduct] describes the behavior of our network
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Events")]
pub struct Mainnet {
    pub freq: ping::Behaviour,
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
}

impl Mainnet {
    pub fn new(
        freq: ping::Behaviour,
        kademlia: Kademlia<MemoryStore>,
        mdns: mdns::tokio::Behaviour,
    ) -> Self {
        Self {
            freq,
            kademlia,
            mdns,
        }
    }
}
