/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use libp2p::{identity::PublicKey, PeerId};

pub trait Peerable: Clone {
    fn pk(self) -> PublicKey;
    fn pid(&self) -> PeerId {
        PeerId::from(self.clone().pk())
    }
}

pub trait Handle<T> {
    type Error;
    type Output: std::convert::From<T>;

    fn handle(&mut self, msg: T) -> Result<Self::Output, Self::Error>;
}

#[async_trait::async_trait]
pub trait AsyncHandle<T: Send + Sync> {
    type Error: Send + Sync;
    type Output: std::convert::From<T>;

    async fn handle(&mut self, msg: T) -> Result<Self::Output, Self::Error>;
}
