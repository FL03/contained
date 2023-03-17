/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Custom Network Events
*/
use libp2p::{kad, mdns, ping};

/// [Events] describes the events considered by the network
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Event {
    Kademlia(kad::KademliaEvent),
    Mdns(mdns::Event),
    Ping(ping::Event),
}

impl From<kad::KademliaEvent> for Event {
    fn from(event: kad::KademliaEvent) -> Self {
        Event::Kademlia(event)
    }
}

impl From<mdns::Event> for Event {
    fn from(event: mdns::Event) -> Self {
        Event::Mdns(event)
    }
}

impl From<ping::Event> for Event {
    fn from(event: ping::Event) -> Self {
        Event::Ping(event)
    }
}
