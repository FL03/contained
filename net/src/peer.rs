/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Peerable;
use libp2p::identity::{ed25519, Keypair};

#[derive(Clone, Debug)]
pub struct Peer {
    keypair: [u8; 64],
}

impl Peer {
    pub fn new(keypair: [u8; 64]) -> Self {
        Self { keypair }
    }
}

impl Default for Peer {
    fn default() -> Self {
        Self::from(ed25519::Keypair::generate())
    }
}

impl Peerable for Peer {
    fn keypair(self) -> Keypair {
        Keypair::Ed25519(ed25519::Keypair::decode(&mut self.clone().keypair).unwrap())
    }
}

impl From<ed25519::Keypair> for Peer {
    fn from(keypair: ed25519::Keypair) -> Self {
        Self::new(keypair.encode())
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
        let kp = Keypair::generate_ed25519();
        // let sk = kp.
        assert_ne!(kp.public(), Peer::default().pk());
        // assert
    }
}
