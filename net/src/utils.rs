/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::BoxedTransport;

use libp2p::{
    core::upgrade,
    identity::{ed25519, error::DecodingError, Keypair},
    mplex, noise,
    swarm::{NetworkBehaviour, Swarm},
    tcp,
};
use libp2p::{PeerId, Transport};

pub fn keypair_from_seed(seed: u8) -> Result<ed25519::SecretKey, DecodingError> {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    let secret_key = ed25519::SecretKey::from_bytes(&mut bytes)?;
    Ok(secret_key)
}

pub fn tokio_swarm<B: NetworkBehaviour>(behaviour: B, keypair: &Keypair) -> Swarm<B> {
    Swarm::with_tokio_executor(
        tokio_transport(keypair, true),
        behaviour,
        PeerId::from(keypair.public()),
    )
}

pub fn tokio_transport(keypair: &Keypair, nodelay: bool) -> BoxedTransport {
    tcp::tokio::Transport::new(tcp::Config::default().nodelay(nodelay))
        .upgrade(upgrade::Version::V1)
        .authenticate(
            noise::NoiseAuthenticated::xx(keypair)
                .expect("Signing libp2p-noise static DH keypair failed."),
        )
        .multiplex(mplex::MplexConfig::new())
        .boxed()
}
