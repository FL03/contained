/*
    Appellation: conduit <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A conduit is defined as a means of transporting or distributing something.

        For us, a conduit is a flexible node capable of assuming a number of different forms.
*/
use crate::{
    net::{tokio_transport, Peerable},
    BoxedTransport,
};
use libp2p::{
    identity::{ed25519, Keypair, PublicKey},
    swarm::{NetworkBehaviour, Swarm},
    PeerId,
};

#[derive(Clone, Debug)]
pub struct Conduit {
    kp: Keypair,
}

impl Conduit {
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

impl From<Keypair> for Conduit {
    fn from(kp: Keypair) -> Self {
        Self { kp }
    }
}

impl TryFrom<u8> for Conduit {
    type Error = libp2p::identity::error::DecodingError;

    fn try_from(seed: u8) -> Result<Self, Self::Error> {
        let mut bytes = [0u8; 32];
        bytes[0] = seed;
        let secret_key = ed25519::SecretKey::from_bytes(&mut bytes)?;
        Ok(Self::from(Keypair::Ed25519(secret_key.into())))
    }
}

impl Peerable for Conduit {
    fn pk(&self) -> PublicKey {
        self.kp.public()
    }
    fn pid(&self) -> PeerId {
        PeerId::from(self.pk())
    }
}

impl Default for Conduit {
    fn default() -> Self {
        Self::new()
    }
}
