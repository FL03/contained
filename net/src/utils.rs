/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{peers::Peer, Conduct};
use libp2p::identity::{self, Keypair};
use libp2p::swarm::SwarmBuilder;

/// [keypair_from_seed] A simple function wrapper attempting to create an [Keypair] from the provided seed
pub fn keypair_from_seed(seed: u8) -> Result<Keypair, identity::DecodingError> {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    Keypair::from_protobuf_encoding(&mut bytes)
}

pub fn swarm<B: Conduct>(peer: Peer) -> libp2p::Swarm<B> {
    let behaviour = B::from_peer(peer.clone());
    SwarmBuilder::with_tokio_executor(peer.transport(), behaviour, peer.pid()).build()
}
