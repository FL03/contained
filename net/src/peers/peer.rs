/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Peerable;
use libp2p::identity::{ed25519, DecodingError, Keypair};

pub struct Keys {
    pk: [u8; 32],
    sk: [u8; 32],
}

impl Keys {
    pub fn new(pk: [u8; 32], sk: [u8; 32]) -> Self {
        Self { pk, sk }
    }
    pub fn public_key(&self) -> [u8; 32] {
        self.pk
    }
    pub fn secret_key(&self) -> [u8; 32] {
        self.sk
    }
}

impl From<Keys> for Keypair {
    fn from(keys: Keys) -> Self {
        let mut pk = keys.pk;
        let mut sk = keys.sk;
        Keypair::Ed25519(ed25519::Keypair::decode_ed25519(&mut pk, &mut sk).unwrap())
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
    type Error = DecodingError;

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
