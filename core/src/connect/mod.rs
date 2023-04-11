/*
    Appellation: connect <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{connection::*, frame::*};

mod connection;
mod frame;

use async_trait::async_trait;
use bytes::Buf;
use tokio::net::ToSocketAddrs;

pub trait TokioFrame {
    type Error;
    fn check(buf: &mut impl Buf) -> Result<(), Self::Error>;
    fn parse(buf: &mut impl Buf) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

#[async_trait]
pub trait AsyncConnection<Frame: TokioFrame> {
    type Error: Send + Sync;

    async fn connect(addr: impl ToSocketAddrs) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn read(&mut self) -> Result<Option<Frame>, Self::Error>;
    async fn write(&mut self, frame: &Frame) -> Result<(), Self::Error>;
}
