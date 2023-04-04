/*
    Appellation: cmd <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: These commands describe the possible operations of a node in the network
*/
use crate::NetworkResult;
use crate::subnet::proto::reqres::{Request, Response};
use libp2p::{Multiaddr, PeerId};
use libp2p::request_response::ResponseChannel;
use std::collections::HashSet;
use tokio::sync::oneshot::Sender;

pub enum Event {
    Listening,
    Dialing,

    Providing,
}

#[derive(Debug)]
pub enum Command {
    StartListening {
        addr: Multiaddr,
        tx: Sender<NetworkResult>,
    },
    Dial {
        addr: Multiaddr,
        pid: PeerId,
        tx: Sender<NetworkResult>,
    },
    GetProviders {
        id: String,
        tx: Sender<HashSet<PeerId>>,
    },
    Request {
        payload: String,
        peer: PeerId,
        tx: Sender<NetworkResult<Response>>,
    },
    Respond {
        payload: Vec<u8>,
        channel: ResponseChannel<Response>,
    },
    StartProviding {
        id: String,
        tx: Sender<()>,
    },
}

impl Command {
    pub fn dial(addr: Multiaddr, pid: PeerId, sender: Sender<NetworkResult>) -> Self {
        Self::Dial { addr, pid, tx: sender }
    }
    pub fn listen(addr: Multiaddr, sender: Sender<NetworkResult>) -> Self {
        Self::StartListening { addr, tx: sender }
    }
    pub fn get_provider(id: String, sender: Sender<HashSet<PeerId>>) -> Self {
        Self::GetProviders { id, tx: sender }
    }
    pub fn request(payload: String, peer: PeerId, sender: Sender<NetworkResult<Response>>) -> Self {
        Self::Request { payload, peer, tx: sender }
    }
    pub fn response(payload: Vec<u8>, channel: ResponseChannel<Response>) -> Self {
        Self::Respond { payload, channel }
    }
    pub fn start_providing(id: String, sender: Sender<()>) -> Self {
        Self::StartProviding { id, tx: sender }
    }
}
