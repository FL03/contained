/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Custom Network Events
*/
use super::proto::reqres::ReqResEvent;
use libp2p::{identify, kad, mdns, ping};

/// [Events] describes the events considered by the network
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum SubnetEvent {
    Identify(identify::Event),
    Kademlia(kad::KademliaEvent),
    Mdns(mdns::Event),
    Ping(ping::Event),
    RequestResponse(ReqResEvent),
}

impl From<identify::Event> for SubnetEvent {
    fn from(event: identify::Event) -> Self {
        Self::Identify(event)
    }
}

impl From<kad::KademliaEvent> for SubnetEvent {
    fn from(event: kad::KademliaEvent) -> Self {
        Self::Kademlia(event)
    }
}

impl From<mdns::Event> for SubnetEvent {
    fn from(event: mdns::Event) -> Self {
        Self::Mdns(event)
    }
}

impl From<ping::Event> for SubnetEvent {
    fn from(event: ping::Event) -> Self {
        Self::Ping(event)
    }
}

impl From<ReqResEvent> for SubnetEvent {
    fn from(event: ReqResEvent) -> Self {
        Self::RequestResponse(event)
    }
}
