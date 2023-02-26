/*
    Appellation: behaviour <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use libp2p::{mdns, ping};
use libp2p::swarm::NetworkBehaviour;


/// [Conduct] describes the behavior of our network
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Events")]
pub struct Conduct {
    freq: ping::Behaviour,
    mdns: mdns::tokio::Behaviour
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

/// [Events] describes how the [Conduct] responds to different events
#[allow(clippy::large_enum_variant)]
pub enum Events {
    Mdns(mdns::Event),
    Ping(ping::Event),
}

impl From<mdns::Event> for Events {
    fn from(event: mdns::Event) -> Self {
        Events::Mdns(event)
    }
}

impl From<ping::Event> for Events {
    fn from(event: ping::Event) -> Self {
        Events::Ping(event)
    }
}
