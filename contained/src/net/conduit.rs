/*
    Appellation: conduit <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A conduit is defined as a means of transporting or distributing something.

        For us, a conduit is a flexible node capable of assuming a number of different forms.
*/
use crate::net::Peerable;
use libp2p::{identity::{Keypair, PublicKey}, Multiaddr, PeerId,};

#[derive(Clone, Debug)]
pub struct Conduit {
    addr: Multiaddr,
    kp: Keypair
}

impl Conduit {
    pub fn new(addr: Multiaddr, kp: Keypair) -> Self {
        Self { addr, kp }
    }
    
    pub fn address(&self) -> Multiaddr {
        self.addr.clone()
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
        Self::new("/ip4/0.0.0.0/tcp/0".parse().unwrap(), Keypair::generate_ed25519())
    }
}
