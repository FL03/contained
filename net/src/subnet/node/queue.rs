/*
    Appellation: queue <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements a queue for the network, which is used to store pending requests.
*/
use crate::subnet::{layer::Command, Subnet};
use crate::NetworkResult;
use libp2p::multiaddr::Protocol;
use libp2p::{kad::QueryId, PeerId, Swarm};
use std::collections::{hash_map, HashMap, HashSet};
use tokio::sync::oneshot::Sender;

/// The queue is a collection of all the pending requests.
#[derive(Debug, Default)]
pub struct Queue {
    pub dial: HashMap<PeerId, Sender<NetworkResult>>,
    pub start_providing: HashMap<QueryId, Sender<()>>,
    pub get_providers: HashMap<QueryId, Sender<HashSet<PeerId>>>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            dial: HashMap::new(),
            start_providing: HashMap::new(),
            get_providers: HashMap::new(),
        }
    }
    pub async fn handle(&mut self, action: Command, swarm: &mut Swarm<Subnet>) {
        match action {
            Command::StartListening { addr, sender } => {
                let _ = match swarm.listen_on(addr) {
                    Ok(_) => sender.send(Ok(())),
                    Err(e) => sender.send(Err(e.into())),
                };
            }
            Command::Dial { addr, pid, sender } => match self.dial.entry(pid) {
                hash_map::Entry::Occupied(_) => {
                    tracing::warn!("The peer ({}) is already being dialed", pid);
                }
                hash_map::Entry::Vacant(e) => {
                    swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(&pid, addr.clone());
                    let dialopts = addr.with(Protocol::P2p((pid).into()));
                    match swarm.dial(dialopts) {
                        Ok(_) => {
                            e.insert(sender);
                        }
                        Err(e) => {
                            let _ = sender.send(Err(e.into()));
                        }
                    }
                }
            },
            Command::StartProviding { .. } => {}
            Command::GetProviders { .. } => {}
        }
    }
}