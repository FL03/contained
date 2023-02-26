/*
    Appellation: behaviour <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use libp2p::ping;
use libp2p::swarm::NetworkBehaviour;
use smart_default::SmartDefault;


#[derive(NetworkBehaviour, SmartDefault)]
#[behaviour(out_event = "Events")]
pub struct ConduitBehaviour {
    ping: ping::Behaviour,
}

#[allow(clippy::large_enum_variant)]
pub enum Events {
    Ping(ping::Event)
}

impl From<ping::Event> for Events {
    fn from(event: ping::Event) -> Self {
        Events::Ping(event)
    }
}
