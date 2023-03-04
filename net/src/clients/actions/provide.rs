/*
    Appellation: dial <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use libp2p::PeerId;
use std::collections::HashSet;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct GetProviders {
    fname: String,
    sender: oneshot::Sender<HashSet<PeerId>>,
}

impl GetProviders {
    pub fn new(fname: String, sender: oneshot::Sender<HashSet<PeerId>>) -> Self {
        Self { fname, sender }
    }
}

#[derive(Debug)]
pub struct StartProviding {
    fname: String,
    sender: oneshot::Sender<()>,
}

impl StartProviding {
    pub fn new(fname: String, sender: oneshot::Sender<()>) -> Self {
        Self { fname, sender }
    }
}
