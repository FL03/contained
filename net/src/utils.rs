/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::BoxedTransport;

use libp2p::{
    core::upgrade,
    identity::Keypair,
    mplex, noise,
    swarm::{NetworkBehaviour, Swarm},
    tcp,
};
use libp2p::{PeerId, Transport};

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
