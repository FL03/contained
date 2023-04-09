/*
    Appellation: peer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::peer::*;

mod peer;

use crate::Conduct;
use libp2p::identity::Keypair;
use libp2p::swarm::SwarmBuilder;
use libp2p::{PeerId, Swarm};

pub fn swarm<B: Conduct + FromPeer>(peer: Peer) -> Swarm<B> {
    let behaviour = B::from_peer(peer.clone());
    SwarmBuilder::with_tokio_executor(peer.transport(), behaviour, peer.pid()).build()
}

pub trait IntoPeer {
    fn into_peer(self) -> Peer;
}

pub trait FromPeer {
    fn from_peer(peer: Peer) -> Self;
}

pub trait FromPeerId {
    fn from_pid(pid: PeerId) -> Self;
}

impl<T> FromPeerId for T
where
    T: From<PeerId>,
{
    fn from_pid(pid: PeerId) -> Self {
        Self::from(pid)
    }
}

pub trait FromKeypair {
    fn from_kp(kp: Keypair) -> Self;
}

impl<T> FromKeypair for T
where
    T: From<Keypair>,
{
    fn from_kp(kp: Keypair) -> Self {
        Self::from(kp)
    }
}
