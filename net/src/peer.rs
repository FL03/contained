/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::BoxedTransport;
use libp2p::swarm::{NetworkBehaviour, Swarm};
use libp2p::{
    core::upgrade,
    identity::{ed25519, Keypair, PublicKey},
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Peer {
    keypair: [u8; 64],
}

impl Peer {
    pub fn new(keypair: [u8; 64]) -> Self {
        Self { keypair }
    }
    pub fn swarm<B: NetworkBehaviour>(&self, behaviour: B) -> Swarm<B> {
        Swarm::with_tokio_executor(self.transport(), behaviour, self.pid())
    }
}

impl Default for Peer {
    fn default() -> Self {
        Self::from(ed25519::Keypair::generate())
    }
}

impl Peerable for Peer {
    fn keypair(self) -> Keypair {
        self.into()
    }
}

impl From<ed25519::Keypair> for Peer {
    fn from(keypair: ed25519::Keypair) -> Self {
        Self::new(keypair.encode())
    }
}

impl From<Peer> for Keypair {
    fn from(peer: Peer) -> Keypair {
        let mut kp = peer.keypair;
        Keypair::Ed25519(ed25519::Keypair::decode(&mut kp).unwrap())
    }
}

impl TryFrom<u8> for Peer {
    type Error = libp2p::identity::error::DecodingError;

    fn try_from(seed: u8) -> Result<Self, Self::Error> {
        let sk = crate::sk_from_seed(seed)?;
        let res = Self::from(ed25519::Keypair::from(sk));
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer() {
        let peer = Peer::try_from(9_u8);
        assert!(peer.is_ok());
        assert_ne!(peer.unwrap(), Peer::default());
    }
}
