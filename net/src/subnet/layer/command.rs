/*
    Appellation: cmd <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: These commands describe the possible operations of a node in the network
*/
use crate::subnet::proto::reqres::Response;
use crate::NetworkResult;
use libp2p::request_response::ResponseChannel;
use libp2p::{Multiaddr, PeerId};
use std::collections::HashSet;
use tokio::sync::oneshot::Sender;

pub type Commander<T = ()> = Sender<NetworkResult<T>>;

pub enum Event {
    Listening,
    Dialing,
    FoundProviders { providers: HashSet<PeerId> },
    Providing,
    Response { res: Response },
}

#[derive(Debug)]
pub enum Command {
    Listen {
        addr: Multiaddr,
        tx: Commander,
    },
    Dial {
        addr: Multiaddr,
        pid: PeerId,
        tx: Commander,
    },
    Provide {
        cid: String,
        tx: Commander,
    },
    Providers {
        cid: String,
        tx: Commander<HashSet<PeerId>>,
    },
    Request {
        payload: String,
        peer: PeerId,
        tx: Commander<Response>,
    },
    Respond {
        payload: Vec<u8>,
        channel: ResponseChannel<Response>,
    },
}

impl Command {
    pub fn dial(addr: Multiaddr, pid: PeerId, tx: Commander) -> Self {
        Self::Dial { addr, pid, tx }
    }
    pub fn listen(addr: Multiaddr, tx: Commander) -> Self {
        Self::Listen { addr, tx }
    }
    pub fn provide(cid: String, tx: Commander) -> Self {
        Self::Provide { cid, tx }
    }
    pub fn providers(cid: String, tx: Commander<HashSet<PeerId>>) -> Self {
        Self::Providers { cid, tx }
    }
    pub fn request(payload: String, peer: PeerId, tx: Commander<Response>) -> Self {
        Self::Request { payload, peer, tx }
    }
    pub fn response(payload: Vec<u8>, channel: ResponseChannel<Response>) -> Self {
        Self::Respond { payload, channel }
    }
}
