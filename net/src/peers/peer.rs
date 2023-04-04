/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Peerable;
use libp2p::identity::{DecodingError, Keypair};

#[derive(Clone, Debug)]
pub struct Peer {
    kp: Keypair,
}

impl Peer {
    pub fn new(kp: Keypair) -> Self {
        Self { kp }
    }
}

impl Default for Peer {
    fn default() -> Self {
        Self::from(Keypair::generate_ed25519())
    }
}

impl Peerable for Peer {
    fn keypair(self) -> Keypair {
        self.into()
    }
}

impl From<Keypair> for Peer {
    fn from(keypair: Keypair) -> Self {
        Self::new(keypair)
    }
}

impl From<Peer> for Keypair {
    fn from(peer: Peer) -> Keypair {
        peer.kp
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
