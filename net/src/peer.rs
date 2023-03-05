/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{tokio_transport, BoxedTransport, Peerable};
use libp2p::{
    identity::{ed25519, Keypair, PublicKey},
    swarm::{NetworkBehaviour, Swarm},
};

#[derive(Clone, Debug)]
pub struct Peer {
    kp: Keypair,
}

impl Peer {
    pub fn new() -> Self {
        let kp = Keypair::generate_ed25519();
        Self::from(kp)
    }

    pub fn swarm<B: NetworkBehaviour>(&self, behaviour: B) -> Swarm<B> {
        Swarm::with_tokio_executor(self.transport(), behaviour, self.pid())
    }

    pub fn transport(&self) -> BoxedTransport {
        tokio_transport(&self.kp, true)
    }
}

impl Default for Peer {
    fn default() -> Self {
        Self::new()
    }
}

impl Peerable for Peer {
    fn pk(self) -> PublicKey {
        self.kp.public()
    }
}

impl From<Keypair> for Peer {
    fn from(kp: Keypair) -> Self {
        Self { kp }
    }
}

impl TryFrom<u8> for Peer {
    type Error = libp2p::identity::error::DecodingError;

    fn try_from(seed: u8) -> Result<Self, Self::Error> {
        let mut bytes = [0u8; 32];
        bytes[0] = seed;
        let secret_key = ed25519::SecretKey::from_bytes(&mut bytes)?;
        Ok(Self::from(Keypair::Ed25519(secret_key.into())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer() {
        let kp = Keypair::generate_ed25519();
        // let sk = kp.
        assert_ne!(kp.public(), Peer::default().pk());
        // assert
    }
}
