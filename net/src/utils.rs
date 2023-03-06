/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use libp2p::identity::{ed25519, error::DecodingError};

/// [sk_from_seed] A simple function wrapper attempting to create an [ed25519::SecretKey] from the provided seed
pub fn sk_from_seed(seed: u8) -> Result<ed25519::SecretKey, DecodingError> {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    let secret_key = ed25519::SecretKey::from_bytes(&mut bytes)?;
    Ok(secret_key)
}
