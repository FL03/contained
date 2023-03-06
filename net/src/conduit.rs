/*
    Appellation: conduit <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A conduit is defined as a means of transporting or distributing something.

        For us, a conduit is a flexible node capable of assuming a number of different forms.
*/
use crate::peer::Peer;
use crate::{BoxedTransport, Peerable};
use libp2p::{
    core::upgrade,
    swarm::{NetworkBehaviour, Swarm},
};
use libp2p::{mplex, noise, tcp, PeerId, Transport};

#[derive(Clone, Debug, Default)]
pub struct Conduit {
    peer: Peer,
}

impl Conduit {
    pub fn new(peer: Peer) -> Self {
        Self { peer }
    }
    ///
    pub fn peer(self) -> Peer {
        self.peer
    }
    ///
    pub fn swarm<B: NetworkBehaviour>(&self, behaviour: B) -> Swarm<B> {
        Swarm::with_tokio_executor(
            self.transport(),
            behaviour,
            PeerId::from(self.clone().peer().pk()),
        )
    }
    ///
    pub fn transport(&self) -> BoxedTransport {
        tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(
                noise::NoiseAuthenticated::xx(&self.clone().peer().keypair())
                    .expect("Signing libp2p-noise static DH keypair failed."),
            )
            .multiplex(mplex::MplexConfig::new())
            .boxed()
    }
}
