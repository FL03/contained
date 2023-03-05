/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{event::*, event_loop::*};

pub(crate) mod event;
pub(crate) mod event_loop;

use libp2p::{kad, mdns, ping};

/// [Events] describes the events considered by the network
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Events {
    Kademlia(kad::KademliaEvent),
    Mdns(mdns::Event),
    Ping(ping::Event),
}

impl From<kad::KademliaEvent> for Events {
    fn from(event: kad::KademliaEvent) -> Self {
        Events::Kademlia(event)
    }
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
