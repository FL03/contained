/*
    Appellation: frame <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Tokio.rs defines a frame to be the unit of data shared between any two peers.

*/
pub use self::{dial::*, listen::*, provide::*};

pub(crate) mod dial;
pub(crate) mod listen;
pub(crate) mod provide;

use crate::NetResult;

use libp2p::{Multiaddr, PeerId};
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Frame {
    StartListening(Listen),
    Dial(Dial),
    StartProviding(StartProviding),
    GetProviders(GetProviders),
}

impl Frame {
    pub fn dial(addr: Multiaddr, pid: PeerId, sender: oneshot::Sender<NetResult>) -> Self {
        Self::from(Dial::new(addr, pid, sender))
    }
    pub fn listen(addr: Multiaddr, sender: oneshot::Sender<NetResult>) -> Self {
        Self::from(Listen::new(addr, sender))
    }
}

impl From<Dial> for Frame {
    fn from(data: Dial) -> Frame {
        Frame::Dial(data)
    }
}

impl From<Listen> for Frame {
    fn from(data: Listen) -> Frame {
        Frame::StartListening(data)
    }
}

impl From<StartProviding> for Frame {
    fn from(data: StartProviding) -> Frame {
        Frame::StartProviding(data)
    }
}

impl From<GetProviders> for Frame {
    fn from(data: GetProviders) -> Frame {
        Frame::GetProviders(data)
    }
}
