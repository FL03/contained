/*
    Appellation: event_loop <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

use crate::NetResult;
use libp2p::kad::QueryId;
use libp2p::PeerId;
use std::collections::{HashMap, HashSet};
use tokio::sync::oneshot;

#[derive(Default)]
pub struct EventLoop {
    dial: HashMap<PeerId, oneshot::Sender<NetResult>>,
    start_providing: HashMap<QueryId, oneshot::Sender<()>>,
    get_providers: HashMap<QueryId, oneshot::Sender<HashSet<PeerId>>>,
}

impl EventLoop {
    pub fn new(
        dial: HashMap<PeerId, oneshot::Sender<NetResult>>,
        start_providing: HashMap<QueryId, oneshot::Sender<()>>,
        get_providers: HashMap<QueryId, oneshot::Sender<HashSet<PeerId>>>,
    ) -> Self {
        Self {
            dial,
            start_providing,
            get_providers,
        }
    }
}
