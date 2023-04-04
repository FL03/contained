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

pub trait TokioFrame {
    type Error;
    fn check(buf: &mut impl Buf) -> Result<(), Self::Error>;
    fn parse(buf: &mut impl Buf) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

#[async_trait]
pub trait Connector {
    type Frame: TokioFrame;
}
