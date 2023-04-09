/*
    Appellation: queue <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements a queue for the network, which is used to store pending requests.
*/
use crate::subnet::proto::reqres;
use crate::NetworkResult;
use libp2p::request_response::RequestId;
use libp2p::{kad::QueryId, PeerId};
use std::collections::{HashMap, HashSet};
use tokio::sync::oneshot::Sender;

/// The queue is a collection of all the pending requests.
#[derive(Debug, Default)]
pub struct Queue {
    pub dial: HashMap<PeerId, Sender<NetworkResult>>,
    pub start_providing: HashMap<QueryId, Sender<()>>,
    pub get_providers: HashMap<QueryId, Sender<HashSet<PeerId>>>,
    pub requests: HashMap<RequestId, Sender<NetworkResult<reqres::Response>>>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            dial: HashMap::new(),
            start_providing: HashMap::new(),
            get_providers: HashMap::new(),
            requests: HashMap::new(),
        }
    }
}
