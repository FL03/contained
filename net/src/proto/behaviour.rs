/*
    Appellation: behaviour <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::events::Events;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{mdns, ping};

/// [Conduct] describes the behavior of our network
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Events")]
pub struct Conduct {
    freq: ping::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

impl Conduct {
    pub fn new(freq: ping::Behaviour, mdns: mdns::tokio::Behaviour) -> Self {
        Self { freq, mdns }
    }
}

impl From<mdns::tokio::Behaviour> for Conduct {
    fn from(b: mdns::tokio::Behaviour) -> Self {
        Self::new(Default::default(), b)
    }
}
