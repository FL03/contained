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
use std::collections::HashSet;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Frame {
    StartListening(Listen),
    Dial(Dial),
    StartProviding(StartProviding),
    GetProviders(GetProviders),
}

impl Frame {
    pub fn dial(addr: Multiaddr, pid: PeerId, sender: Sender<NetResult>) -> Self {
        Dial::new(addr, pid, sender).into()
    }
    pub fn listen(addr: Multiaddr, sender: Sender<NetResult>) -> Self {
        Listen::new(addr, sender).into()
    }
    pub fn get(fname: String, sender: Sender<HashSet<PeerId>>) -> Self {
        GetProviders::new(fname, sender).into()
    }
    pub fn provide(fname: String, sender: Sender<()>) -> Self {
        StartProviding::new(fname, sender).into()
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
