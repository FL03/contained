/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Peerable;
use libp2p::identity::{DecodingError, Keypair};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
        Self::new(keypair.into_ed25519().unwrap().encode())
    }
}

impl From<Peer> for Keypair {
    fn from(peer: Peer) -> Keypair {
        let mut kp = peer.keypair;
        Keypair::from_protobuf_encoding(&mut kp).unwrap()
    }
}

impl TryFrom<u8> for Peer {
    type Error = DecodingError;

    fn try_from(seed: u8) -> Result<Self, Self::Error> {
        let mut bytes = [0u8; 32];
        bytes[0] = seed;
        let res = Self::from(Keypair::ed25519_from_bytes(&mut bytes)?);
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
