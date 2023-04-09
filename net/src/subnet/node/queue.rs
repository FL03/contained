/*
    Appellation: queue <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements a queue for the network, which is used to store pending workloads
*/
use crate::subnet::{
    layer::Commander,
    proto::reqres::{self, RequestId},
};
use libp2p::{kad::QueryId, PeerId};
use std::collections::{HashMap, HashSet};

/// The queue is a collection of all the pending requests.
#[derive(Debug, Default)]
pub struct Queue {
    pub dial: HashMap<PeerId, Commander>,
    pub start_providing: HashMap<QueryId, Commander>,
    pub get_providers: HashMap<QueryId, Commander<HashSet<PeerId>>>,
    pub requests: HashMap<RequestId, Commander<reqres::Response>>,
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
