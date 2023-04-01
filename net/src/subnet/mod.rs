/*
    Appellation: subnet <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Subnets describe user owned clusters of nodes. Subnets are used to provide a secure and private environment for the execution of various workloads and services.
*/
pub use self::events::*;

mod events;
pub mod reqres;

use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::swarm::NetworkBehaviour;
use libp2p::{mdns, ping, PeerId};

/// [Conduct] describes the behavior of our network
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "SubnetEvent")]
pub struct Subnet {
    pub freq: ping::Behaviour,
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
}

impl Subnet {
    pub fn new(kademlia: Kademlia<MemoryStore>, mdns: mdns::tokio::Behaviour) -> Self {
        Self {
            freq: Default::default(),
            kademlia,
            mdns,
        }
    }
}

impl From<PeerId> for Subnet {
    fn from(pid: PeerId) -> Self {
        let kademlia = Kademlia::new(pid, MemoryStore::new(pid));
        Self::new(
            kademlia,
            mdns::tokio::Behaviour::new(mdns::Config::default(), pid).unwrap(),
        )
    }
}
