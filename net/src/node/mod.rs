/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the node for the clusters / subnets. A node is best understood as a fragmented tonnetz capable of being glued together with other eligble nodes.
        With this in mind, it is important to consider that the mainnet considers the interactions between engaging entities or clusters.
        The mainnet is a virtual overlay network designed for efficient and secure communication between clusters, permitting the exchange of data and information between users,
        and for persisting information, resources, and otherwirse. In order to support the mainnet, users typically allocate a set amount of resources or specify a certain device in their personal cloud
        for the network to leverage. These partitions are cryptographically secure and prevent the user from accessing the contents of the device once toggled.

        Subnets or clusters are made up of physical nodes, optimized for the execution of various workloads and services. Each device registered to the system is partitioned into a set of locally persisted
        triads,
*/
pub use self::queue::*;

mod queue;

use crate::events::NetworkEvent;
use crate::peers::{Peer, Peerable};
use crate::subnet::{layer::Command, Subnet, SubnetEvent};
use crate::NetworkResult;
use futures::StreamExt;
use libp2p::kad::{self, KademliaEvent, QueryResult};
use libp2p::multiaddr::Protocol;
use libp2p::request_response;
use libp2p::swarm::{SwarmEvent, THandlerErr};
use libp2p::{mdns, Swarm};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct Channels {
    pub cmd: mpsc::Receiver<Command>,
    pub event: mpsc::Sender<NetworkEvent>,
}

impl Channels {
    pub fn new(capacity: usize) -> Self {
        Self {
            cmd: mpsc::channel(capacity).1,
            event: mpsc::channel(capacity).0,
        }
    }
}

impl Default for Channels {
    fn default() -> Self {
        Self::new(9)
    }
}

pub struct Node {
    chan: Channels,
    queue: Queue,
    swarm: Swarm<Subnet>,
}

impl Node {
    pub fn new(swarm: Swarm<Subnet>) -> Self {
        Self {
            chan: Channels::default(),
            queue: Queue::new(),
            swarm,
        }
    }
    pub async fn handle_event(&mut self, event: SwarmEvent<SubnetEvent, THandlerErr<Subnet>>) {
        match event {
            // Handle custom networking events
            SwarmEvent::Behaviour(b) => match b {
                SubnetEvent::Kademlia(k) => match k {
                    KademliaEvent::OutboundQueryProgressed { id, result, .. } => match result {
                        QueryResult::GetProviders(Ok(get_providers)) => match get_providers {
                            kad::GetProvidersOk::FoundProviders { providers, .. } => {
                                if let Some(sender) = self.queue.get_providers.remove(&id) {
                                    sender.send(providers).expect("Receiver not to be dropped");

                                    // Finish the query. We are only interested in the first result.
                                    self.swarm
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
                                .queue
                                .start_providing
                                .remove(&id)
                                .expect("Completed query to be previously pending.");
                            let _ = sender.send(());
                        }
                        _ => {}
                    },
                    _ => {}
                },
                SubnetEvent::Mdns(mdns_event) => match mdns_event {
                    mdns::Event::Discovered(_disc) => {}
                    mdns::Event::Expired(_exp) => {}
                },
                SubnetEvent::Ping(_) => {}
                SubnetEvent::RequestResponse(evnt) => match evnt {
                    request_response::Event::Message { .. } => todo!(),
                    request_response::Event::OutboundFailure { .. } => todo!(),
                    request_response::Event::InboundFailure { .. } => todo!(),
                    request_response::Event::ResponseSent { .. } => todo!(),
                },
            },
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                if let libp2p::core::ConnectedPoint::Dialer { .. } = endpoint {
                    if let Some(sender) = self.queue.dial.remove(&peer_id) {
                        let _ = sender.send(Ok(()));
                    }
                }
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {
                if let Some(pid) = peer_id {
                    if let Some(sender) = self.queue.dial.remove(&pid) {
                        let _ = sender.send(Err(error.into()));
                    }
                }
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                let local_peer_id = *self.swarm.local_peer_id();
                eprintln!(
                    "Local node is listening on {:?}",
                    address.with(Protocol::P2p(local_peer_id.into()))
                );
            }
            SwarmEvent::Dialing(pid) => {
                eprintln!("Dialing {pid}")
            }
            SwarmEvent::ConnectionClosed { .. } => {}
            SwarmEvent::IncomingConnection { .. } => {}
            SwarmEvent::IncomingConnectionError { .. } => {}
            SwarmEvent::ExpiredListenAddr { .. } => {}
            SwarmEvent::ListenerClosed { .. } => {}
            // SwarmEvent::ListenerError { .. } => {},
            e => tracing::warn!("Unhandled swarm event: {:?}", e),
        }
    }
    pub async fn run(mut self) -> NetworkResult {
        loop {
            tokio::select! {
                Some(event) = self.swarm.next() => {
                    self.handle_event(event).await;
                }
                Some(cmd) = self.chan.cmd.recv() => {
                    self.queue.handle(cmd, &mut self.swarm).await;
                }

            }
        }
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<NetworkResult> {
        tokio::spawn(self.run())
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::from(Peer::default())
    }
}

impl<P> From<P> for Node
where
    P: Peerable,
{
    fn from(peer: P) -> Self {
        Self::new(peer.swarm(Subnet::from(peer.pid())))
    }
}
