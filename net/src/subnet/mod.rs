/*
    Appellation: subnet <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Subnets describe user owned clusters of nodes. Subnets are used to provide a secure and private environment for the execution of various workloads and services.
*/
pub use self::{client::*, events::*};

mod client;
mod events;

pub mod layer;
pub mod node;
pub mod proto;

use crate::peers::*;
use crate::Conduct;
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::swarm::NetworkBehaviour;
use libp2p::{identity::Keypair, mdns, ping, PeerId};

/// [Subnet] describes the behaviour of a user owned cluster of nodes.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "SubnetEvent")]
pub struct Subnet {
    pub freq: ping::Behaviour,
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
    pub reqres: proto::reqres::ProtoBehaviour,
}

impl Subnet {
    pub fn new(kademlia: Kademlia<MemoryStore>, mdns: mdns::tokio::Behaviour) -> Self {
        Self {
            freq: Default::default(),
            kademlia,
            mdns,
            reqres: proto::reqres::new(),
        }
    }
}

impl Default for Subnet {
    fn default() -> Self {
        Self::from_peer(Peer::default())
    }
}

impl Conduct for Subnet {}

impl From<PeerId> for Subnet {
    fn from(pid: PeerId) -> Self {
        let kademlia = Kademlia::new(pid, MemoryStore::new(pid));
        Self::new(
            kademlia,
            mdns::tokio::Behaviour::new(mdns::Config::default(), pid).unwrap(),
        )
    }
}

impl FromPeer for Subnet {
    fn from_peer(peer: Peer) -> Self {
        Self::from(peer.pid())
    }
}

impl From<Keypair> for Subnet {
    fn from(kp: Keypair) -> Self {
        let pk = kp.public();
        Self::from(pk.to_peer_id())
    }
}
