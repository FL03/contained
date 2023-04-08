/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::FromPeer;
use crate::{BoxedTransport, Conduct};
use libp2p::{core::upgrade, mplex, noise, tcp, PeerId, Transport,};
use libp2p::identity::{DecodingError, Keypair, PublicKey};
use libp2p::swarm::{Swarm, SwarmBuilder};

#[derive(Clone, Debug)]
pub struct Peer {
    kp: Keypair,
}

impl Peer {
    pub fn new(kp: Keypair) -> Self {
        Self { kp }
    }
    pub fn keypair(self) -> Keypair {
        self.kp
    }
    pub fn pk(self) -> PublicKey {
        self.keypair().public()
    }
    pub fn pid(&self) -> PeerId {
        self.clone().pk().to_peer_id()
    }
    pub fn swarm<B: FromPeer + Conduct>(&self) -> Swarm<B> {
        let behaviour = B::from_peer(self.clone());
        SwarmBuilder::with_tokio_executor(self.transport(), behaviour, self.pid()).build()
    }
    ///
    pub fn transport(&self) -> BoxedTransport {
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

impl Default for Peer {
    fn default() -> Self {
        Self {
            kp: Keypair::generate_ed25519(),
        }
    }
}

impl From<Keypair> for Peer {
    fn from(keypair: Keypair) -> Self {
        Self::new(keypair)
    }
}

impl TryFrom<u8> for Peer {
    type Error = DecodingError;

    fn try_from(seed: u8) -> Result<Self, Self::Error> {
        let mut bytes = [0u8; 32];
        bytes[0] = seed;
        let res = Self::new(Keypair::ed25519_from_bytes(&mut bytes)?);
        Ok(res)
    }
}

impl From<Peer> for Keypair {
    fn from(peer: Peer) -> Keypair {
        peer.kp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer() {
        let peer = Peer::try_from(9_u8);
        assert!(peer.is_ok());
        assert_ne!(peer.unwrap().pk(), Peer::default().pk());
    }
}
