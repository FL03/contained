/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{event::*, event_loop::*};

pub(crate) mod event;
pub(crate) mod event_loop;

use libp2p::{mdns, ping};

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
