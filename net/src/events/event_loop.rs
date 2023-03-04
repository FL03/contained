/*
    Appellation: event_loop <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Event;
use crate::{clients::actions::Action, proto::Conduct, NetResult};
use libp2p::kad::QueryId;
use libp2p::{PeerId, Swarm};
use std::collections::{HashMap, HashSet};
use tokio::sync::{mpsc, oneshot};

#[derive(Default)]
pub struct Pending {
    dial: HashMap<PeerId, oneshot::Sender<NetResult>>,
    start_providing: HashMap<QueryId, oneshot::Sender<()>>,
    get_providers: HashMap<QueryId, oneshot::Sender<HashSet<PeerId>>>,
}

impl Pending {
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

pub struct EventLoop {
    swarm: Swarm<Conduct>,
    command_receiver: mpsc::Receiver<Action>,
    event_sender: mpsc::Sender<Event>,
    pending_dial: HashMap<PeerId, oneshot::Sender<NetResult>>,
    pending_start_providing: HashMap<QueryId, oneshot::Sender<()>>,
    pending_get_providers: HashMap<QueryId, oneshot::Sender<HashSet<PeerId>>>,
}

impl EventLoop {
    pub fn new(
        swarm: Swarm<Conduct>,
        command_receiver: mpsc::Receiver<Action>,
        event_sender: mpsc::Sender<Event>,
    ) -> Self {
        Self {
            swarm,
            command_receiver,
            event_sender,
            pending_dial: Default::default(),
            pending_start_providing: Default::default(),
            pending_get_providers: Default::default(),
        }
    }
}
