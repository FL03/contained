/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use libp2p::{
    identity::{Keypair, PublicKey},
    PeerId,
};

pub trait Peerable: Clone {
    fn keypair(self) -> Keypair;
    fn pk(self) -> PublicKey {
        self.clone().keypair().public()
    }
    fn pid(&self) -> PeerId {
        self.clone().pk().to_peer_id()
    }
}

pub trait Handle<T> {
    type Output;

    fn handle(&mut self, msg: T) -> Self::Output;
}

#[async_trait::async_trait]
pub trait AsyncHandle<T: Send + Sync> {
    type Output: Send + Sync;

    async fn handle(&mut self, msg: T) -> Self::Output;
}
