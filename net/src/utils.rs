/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use libp2p::identity::{DecodingError, Keypair};

/// [keypair_from_seed] A simple function wrapper attempting to create an [Keypair] from the provided seed
pub fn keypair_from_seed(seed: u8) -> Result<Keypair, DecodingError> {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    Keypair::from_protobuf_encoding(&mut bytes)
}
