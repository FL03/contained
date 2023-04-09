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
use libp2p::{identify, identity::Keypair, mdns, ping};

/// [Subnet] describes the behaviour of a user owned cluster of nodes.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "SubnetEvent")]
pub struct Subnet {
    pub identify: identify::Behaviour,
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour,
    pub reqres: proto::reqres::ProtoBehaviour,
}

impl Subnet {
    pub fn new(peer: Peer) -> Self {
        let pid = peer.pid();
        let identify = identify::Behaviour::new(identify::Config::new(
            "/flow/id/0.0.1".to_string(),
            peer.pk(),
        ));
        let kademlia = Kademlia::new(pid, MemoryStore::new(pid));
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), pid).unwrap();
        Self {
            identify,
            kademlia,
            mdns,
            ping: Default::default(),
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

impl FromPeer for Subnet {
    fn from_peer(peer: Peer) -> Self {
        Self::new(peer)
    }
}

impl From<Keypair> for Subnet {
    fn from(kp: Keypair) -> Self {
        Self::from_peer(Peer::new(kp))
    }
}
