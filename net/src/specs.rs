/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::peers::FromPeer;
use async_trait::async_trait;
use libp2p::identity::{Keypair, PublicKey};
use libp2p::swarm::NetworkBehaviour;
use libp2p::PeerId;

#[async_trait]
pub trait AsyncHandle<T: Send + Sync> {
    type Output: Send + Sync;

    async fn handle(&mut self, msg: T) -> Self::Output;
}

pub trait Handle<T> {
    type Output;

    fn handle(&mut self, msg: T) -> Self::Output;
}

pub trait Conduct: FromPeer + NetworkBehaviour {}

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

pub trait FromPublicKey {
    fn from_pk(pk: PublicKey) -> Self;
}

impl<T> FromPublicKey for T
where
    T: From<PublicKey>,
{
    fn from_pk(pk: PublicKey) -> Self {
        Self::from(pk)
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
