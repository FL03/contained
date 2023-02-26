/*
    Appellation: conduit <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A conduit is defined as a means of transporting or distributing something.
*/
use libp2p::swarm::{keep_alive, NetworkBehaviour, Swarm, SwarmEvent};
use libp2p::ping;
use smart_default::SmartDefault;

#[derive(NetworkBehaviour, SmartDefault)]
pub struct Conduit {
    frequency: keep_alive::Behaviour,
    ping: ping::Behaviour,
}
