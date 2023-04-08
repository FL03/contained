/*
    Appellation: layer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: These commands describe the possible operations of a node in the network
*/
pub use self::command::*;

mod command;

use libp2p::{Multiaddr, PeerId};

pub enum Action {
    Dial { addr: Multiaddr, pid: PeerId },
    Listen { addr: Multiaddr },
    Provide { cid: String },
    Providers { cid: String },
    Request { payload: String, peer: PeerId },
    Respond { payload: Vec<u8> },
}

pub enum Reaction {}

pub enum Layer {
    Application,
    Transport,
    Network,
    Link,
    Physical,
}

pub struct Message<T = String>
where
    T: AsRef<[u8]>,
{
    pub payload: Option<T>,
}

impl<T> Message<T>
where
    T: AsRef<[u8]>,
{
    pub fn new(payload: Option<T>) -> Self {
        Self { payload }
    }
}

impl<T> AsRef<[u8]> for Message<T>
where
    T: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        match &self.payload {
            Some(payload) => payload.as_ref(),
            None => &[],
        }
    }
}
