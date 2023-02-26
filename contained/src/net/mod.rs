/*
    Appellation: net <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{cluster::*, conduit::*};

pub(crate) mod cluster;
pub(crate) mod conduit;

use crate::BoxedTransport;
use anyhow::Result;
use libp2p::{core::upgrade, identity, mplex, noise, swarm::Swarm, tcp};
use libp2p::{Multiaddr, PeerId, Transport};

pub trait Network {
    type Behaviour: libp2p::swarm::NetworkBehaviour;

    fn keypair(&self) -> &identity::Keypair;
    fn pid(&self) -> PeerId {
        PeerId::from(self.keypair().public())
    }
    fn swarm(&self, behaviour: Self::Behaviour) -> Swarm<Self::Behaviour> {
        Swarm::with_tokio_executor(self.transport(), behaviour, self.pid())
    }
    fn transport(&self) -> BoxedTransport {
        tokio_transport(self.keypair(), true)
    }
}

pub fn tokio_transport(keypair: &identity::Keypair, nodelay: bool) -> BoxedTransport {
    tcp::tokio::Transport::new(tcp::Config::default().nodelay(nodelay))
        .upgrade(upgrade::Version::V1)
        .authenticate(
            noise::NoiseAuthenticated::xx(keypair)
                .expect("Signing libp2p-noise static DH keypair failed."),
        )
        .multiplex(mplex::MplexConfig::new())
        .boxed()
}

