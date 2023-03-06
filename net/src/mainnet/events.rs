/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Custom Network Events
*/
use libp2p::{kad, mdns, ping};

/// [Events] describes the events considered by the network
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum NetworkEvent {
    Kademlia(kad::KademliaEvent),
    Mdns(mdns::Event),
    Ping(ping::Event),
}

impl From<kad::KademliaEvent> for NetworkEvent {
    fn from(event: kad::KademliaEvent) -> Self {
        NetworkEvent::Kademlia(event)
    }
}

impl From<mdns::Event> for NetworkEvent {
    fn from(event: mdns::Event) -> Self {
        NetworkEvent::Mdns(event)
    }
}

impl From<ping::Event> for NetworkEvent {
    fn from(event: ping::Event) -> Self {
        NetworkEvent::Ping(event)
    }
}
