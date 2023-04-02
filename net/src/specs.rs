/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use async_trait::async_trait;
use libp2p::swarm::NetworkBehaviour;

#[async_trait]
pub trait AsyncHandle<T: Send + Sync> {
    type Output: Send + Sync;

    async fn handle(&mut self, msg: T) -> Self::Output;
}

pub trait Handle<T> {
    type Output;

    fn handle(&mut self, msg: T) -> Self::Output;
}

pub trait Conduct: NetworkBehaviour {}
