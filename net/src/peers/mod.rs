/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::peer::*;

pub(crate) mod peer;

use crate::BoxedTransport;
use libp2p::swarm::{NetworkBehaviour, Swarm};
use libp2p::{
    core::upgrade,
    identity::{Keypair, PublicKey},
    mplex, noise, tcp, PeerId, Transport,
};

pub trait Peerable: Clone {
    fn keypair(self) -> Keypair;
    fn pk(self) -> PublicKey {
        self.keypair().public()
    }
    fn pid(&self) -> PeerId {
        self.clone().pk().to_peer_id()
    }
    fn swarm<B: NetworkBehaviour>(&self, behaviour: B) -> Swarm<B> {
        Swarm::with_tokio_executor(self.transport(), behaviour, self.pid())
    }
    ///
    fn transport(&self) -> BoxedTransport {
        tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(
                noise::NoiseAuthenticated::xx(&self.clone().keypair())
                    .expect("Signing libp2p-noise static DH keypair failed."),
            )
            .multiplex(mplex::MplexConfig::new())
            .boxed()
    }
}
