/*
    Appellation: acts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{dial::*, listen::*, provide::*};

pub(crate) mod dial;
pub(crate) mod listen;
pub(crate) mod provide;

use crate::NetResult;

use libp2p::{Multiaddr, PeerId};
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Action {
    StartListening(Listen),
    Dial(Dial),
    StartProviding(StartProviding),
    GetProviders(GetProviders),
}

impl Action {
    pub fn dial(addr: Multiaddr, pid: PeerId, sender: oneshot::Sender<NetResult>) -> Self {
        Self::from(Dial::new(addr, pid, sender))
    }
}

impl From<Dial> for Action {
    fn from(data: Dial) -> Action {
        Action::Dial(data)
    }
}

impl From<Listen> for Action {
    fn from(data: Listen) -> Action {
        Action::StartListening(data)
    }
}

impl From<StartProviding> for Action {
    fn from(data: StartProviding) -> Action {
        Action::StartProviding(data)
    }
}

impl From<GetProviders> for Action {
    fn from(data: GetProviders) -> Action {
        Action::GetProviders(data)
    }
}
