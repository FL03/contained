/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This modules implements the network runtime;
*/
use super::{cmd::Command, event_loop::Executor};
use crate::{
    events::NetworkEvent,
    mainnet::{Event, Mainnet},
};
use libp2p::kad::{self, KademliaEvent, QueryResult};
use libp2p::multiaddr::Protocol;
use libp2p::swarm::{SwarmEvent, THandlerErr};
use libp2p::{mdns, Swarm};
use std::collections::hash_map;
use tokio::sync::{mpsc, oneshot};

pub struct Runtime {
    pub(crate) action: mpsc::Receiver<Command>,
    event: mpsc::Sender<NetworkEvent>,
    pub(crate) exec: Executor,
}

impl Runtime {
    pub fn new(
        action: mpsc::Receiver<Command>,
        event: mpsc::Sender<NetworkEvent>,
        exec: Executor,
    ) -> Self {
        Self {
            action,
            event,
            exec,
        }
    }
    pub fn action(self) -> mpsc::Receiver<Command> {
        self.action
    }
    pub fn event(self) -> mpsc::Sender<NetworkEvent> {
        self.event
    }
    pub async fn handle_command(&mut self, action: Command, swarm: &mut Swarm<Mainnet>) {
        match action {
            Command::StartListening { addr, sender } => {
                let _ = match swarm.listen_on(addr) {
                    Ok(_) => sender.send(Ok(())),
                    Err(e) => sender.send(Err(Box::new(e))),
                };
            }
            Command::Dial { addr, pid, sender } => match self.exec.dial.entry(pid) {
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
                            let _ = sender.send(Err(Box::new(e)));
                        }
                    }
                }
            },
            Command::StartProviding { .. } => {}
            Command::GetProviders { .. } => {}
        }
    }
    pub async fn handle_event(
        &mut self,
        event: SwarmEvent<Event, THandlerErr<Mainnet>>,
        swarm: &mut Swarm<Mainnet>,
    ) {
        match event {
            // Handle custom networking events
            SwarmEvent::Behaviour(b) => match b {
                Event::Kademlia(k) => match k {
                    KademliaEvent::OutboundQueryProgressed { id, result, .. } => match result {
                        QueryResult::GetProviders(Ok(get_providers)) => match get_providers {
                            kad::GetProvidersOk::FoundProviders { providers, .. } => {
                                if let Some(sender) = self.exec.get_providers.remove(&id) {
                                    sender.send(providers).expect("Receiver not to be dropped");

                                    // Finish the query. We are only interested in the first result.
                                    swarm
                                        .behaviour_mut()
                                        .kademlia
                                        .query_mut(&id)
                                        .unwrap()
                                        .finish();
                                }
                            }
                            kad::GetProvidersOk::FinishedWithNoAdditionalRecord { .. } => {}
                        },
                        QueryResult::StartProviding(_) => {
                            let sender: oneshot::Sender<()> = self
                                .exec
                                .start_providing
                                .remove(&id)
                                .expect("Completed query to be previously pending.");
                            let _ = sender.send(());
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Event::Mdns(mdns_event) => match mdns_event {
                    mdns::Event::Discovered(_disc) => {}
                    mdns::Event::Expired(_exp) => {}
                },
                Event::Ping(_) => {}
            },
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                if let libp2p::core::ConnectedPoint::Dialer { .. } = endpoint {
                    if let Some(sender) = self.exec.dial.remove(&peer_id) {
                        let _ = sender.send(Ok(()));
                    }
                }
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {
                if let Some(pid) = peer_id {
                    if let Some(sender) = self.exec.dial.remove(&pid) {
                        let _ = sender.send(Err(Box::new(error)));
                    }
                }
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                let local_peer_id = *swarm.local_peer_id();
                eprintln!(
                    "Local node is listening on {:?}",
                    address.with(Protocol::P2p(local_peer_id.into()))
                );
            }
            SwarmEvent::Dialing(pid) => {
                eprintln!("Dialing {pid}")
            }
            SwarmEvent::BannedPeer { .. } => {}
            SwarmEvent::ConnectionClosed { .. } => {}
            SwarmEvent::IncomingConnection { .. } => {}
            SwarmEvent::IncomingConnectionError { .. } => {}
            SwarmEvent::ExpiredListenAddr { .. } => {}
            SwarmEvent::ListenerClosed { .. } => {}
            // SwarmEvent::ListenerError { .. } => {},
            e => panic!("{e:?}"),
        }
    }
    pub fn pending(self) -> Executor {
        self.exec
    }
}
