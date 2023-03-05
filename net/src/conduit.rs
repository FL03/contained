/*
    Appellation: conduit <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A conduit is defined as a means of transporting or distributing something.

        For us, a conduit is a flexible node capable of assuming a number of different forms.
*/
use crate::{tokio_transport, BoxedTransport};
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
        Swarm::with_tokio_executor(self.transport(), behaviour, PeerId::from(self.kp.public()))
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

impl Default for Conduit {
    fn default() -> Self {
        Self::new()
    }
}
