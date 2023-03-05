/*
    Appellation: subnet <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: subnets are optimized for preformance and are essential for the overall success of the framework.
        Subnets are personal clouds, enabling users to orchestrate their workloads across a range of devices.
        Each subnet is considered to be a node for the mainnet.
*/
use crate::events::Events;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{mdns, ping};

/// [Conduct] describes the behavior of our network
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Events")]
pub struct Subnet {
    freq: ping::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

impl Subnet {
    pub fn new(freq: ping::Behaviour, mdns: mdns::tokio::Behaviour) -> Self {
        Self { freq, mdns }
    }
}

impl From<mdns::tokio::Behaviour> for Subnet {
    fn from(b: mdns::tokio::Behaviour) -> Self {
        Self::new(Default::default(), b)
    }
}
