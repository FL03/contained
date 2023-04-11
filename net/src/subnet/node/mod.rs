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
pub use self::{channels::*, queue::*};

mod channels;
mod queue;

use super::{layer::Command, proto::reqres, Subnet, SubnetEvent};
use crate::events::NetworkEvent;
use crate::{FromPeer, NetworkResult, Peer};
use futures::StreamExt;
use libp2p::core::transport::ListenerId;
use libp2p::kad::{self, KademliaEvent, QueryResult};
use libp2p::swarm::{SwarmEvent, THandlerErr};
use libp2p::{identify, mdns, request_response};
use libp2p::{multiaddr::Protocol, Multiaddr, PeerId, Swarm};
use std::collections::hash_map::Entry;

pub struct Node {
    chan: Channels,
    queue: Queue,
    swarm: Swarm<Subnet>,
}

impl Node {
    pub fn new(chan: Channels, swarm: Swarm<Subnet>) -> Self {
        Self {
            chan,
            queue: Queue::new(),
            swarm,
        }
    }
    pub fn dial(&mut self, addr: Multiaddr, pid: PeerId) -> NetworkResult {
        let opts = addr.with(Protocol::P2p((pid).into()));
        self.swarm.dial(opts)?;
        Ok(())
    }
    pub async fn handle_command(&mut self, cmd: Command) -> NetworkResult {
        match cmd {
            Command::Dial { addr, pid, tx } => match self.queue.dial.entry(pid) {
                Entry::Occupied(_) => {
                    tracing::warn!("The peer ({}) is already being dialed", pid);
                }
                Entry::Vacant(entry) => {
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(&pid, addr.clone());
                    let dialopts = addr.with(Protocol::P2p((pid).into()));
                    match self.swarm.dial(dialopts) {
                        Err(e) => {
                            let _ = tx.send(Err(e.into()));
                        }
                        Ok(_) => {
                            entry.insert(tx);
                        }
                    }
                }
            },
            Command::Listen { addr, tx } => {
                let msg = self.swarm.listen_on(addr).map_err(|e| e.into());
                tracing::info!("Listening on {:?}", msg);
                tx.send(msg).expect("Receiver to be still open.");
            }
            Command::Provide { cid, tx } => {
                let key = kad::record::Key::new(&cid.as_bytes());
                let query_id = self
                    .swarm
                    .behaviour_mut()
                    .kademlia
                    .start_providing(key)
                    .expect("Kademlia to be running");
                self.queue.start_providing.insert(query_id, tx);
            }
            Command::Providers { .. } => {}
            Command::Request { payload, peer, tx } => {
                let req = reqres::Request::new(payload);
                let request_id = self.swarm.behaviour_mut().reqres.send_request(&peer, req);
                self.queue.requests.insert(request_id, tx);
            }
            Command::Respond { payload, channel } => {
                let res = reqres::Response::new().with_data(payload);
                self.swarm
                    .behaviour_mut()
                    .reqres
                    .send_response(channel, res)
                    .expect("Connection to peer to be still open.");
            }
        };
        Ok(())
    }
    /// Handle events from the swarm; the stateful network manager
    pub async fn handle_event(
        &mut self,
        event: SwarmEvent<SubnetEvent, THandlerErr<Subnet>>,
    ) -> NetworkResult {
        match event {
            // Handle custom networking events
            SwarmEvent::Behaviour(subnet) => match subnet {
                SubnetEvent::Identify(identify) => match identify {
                    identify::Event::Received { peer_id, .. } => {
                        tracing::info!("Identified peer: {}", peer_id);
                    }
                    e => tracing::warn!("Unhandled identify event: {:?}", e),
                },
                SubnetEvent::Kademlia(kademlia) => match kademlia {
                    KademliaEvent::OutboundQueryProgressed { id, result, .. } => match result {
                        QueryResult::GetProviders(Ok(get_providers)) => match get_providers {
                            kad::GetProvidersOk::FoundProviders { providers, .. } => {
                                if let Some(tx) = self.queue.get_providers.remove(&id) {
                                    tx.send(Ok(providers)).expect("Receiver not to be dropped");
                                    // Finish the query. We are only interested in the first result.
                                    self.swarm
                                        .behaviour_mut()
                                        .kademlia
                                        .query_mut(&id)
                                        .unwrap()
                                        .finish();
                                }
                            }
                            e => tracing::warn!("Unhandled get providers result: {:?}", e),
                        },
                        QueryResult::StartProviding(_) => {
                            let sender = self
                                .queue
                                .start_providing
                                .remove(&id)
                                .expect("Completed query to be previously pending.");
                            let _ = sender.send(Ok(()));
                        }
                        e => tracing::warn!("Unhandled query result: {:?}", e),
                    },
                    _ => {}
                },
                SubnetEvent::Mdns(mdns) => match mdns {
                    mdns::Event::Discovered(disc) => {
                        for (pid, addr) in disc {
                            tracing::info!("Discovered peer: {} at {}", pid, addr);
                        }
                    }
                    e => tracing::warn!("Unhandled mdns event: {:?}", e),
                },
                SubnetEvent::RequestResponse(reqres) => match reqres {
                    request_response::Event::Message { message, .. } => match message {
                        request_response::Message::Request {
                            request, channel, ..
                        } => {
                            self.chan
                                .event()
                                .send(NetworkEvent::inbound_request(request, channel))
                                .await
                                .expect("Receiver not to be dropped");
                        }
                        request_response::Message::Response {
                            response,
                            request_id,
                        } => {
                            let _ = self
                                .queue
                                .requests
                                .remove(&request_id)
                                .expect("pending...")
                                .send(Ok(response));
                        }
                    },
                    request_response::Event::OutboundFailure {
                        request_id, error, ..
                    } => {
                        let _ = self
                            .queue
                            .requests
                            .remove(&request_id)
                            .expect("pending...")
                            .send(Err(error.into()));
                    }
                    request_response::Event::InboundFailure {
                        request_id, error, ..
                    } => {
                        let _ = self
                            .queue
                            .requests
                            .remove(&request_id)
                            .expect("pending...")
                            .send(Err(error.into()));
                    }
                    request_response::Event::ResponseSent { .. } => todo!(),
                },
                e => tracing::warn!("Unhandled subnet event: {:?}", e),
            },
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                if let libp2p::core::ConnectedPoint::Dialer { .. } = endpoint {
                    if let Some(tx) = self.queue.dial.remove(&peer_id) {
                        tx.send(Ok(())).expect("Receiver not to be dropped");
                    }
                }
            }
            SwarmEvent::Dialing(pid) => {
                tracing::info!("Dialing peer: {}", pid);
            }
            SwarmEvent::NewListenAddr { address, .. } => {
                let pid = *self.swarm.local_peer_id();
                tracing::info!(
                    "Local node is listening on {:?}",
                    address.with(Protocol::P2p(pid.into()))
                );
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {
                if let Some(pid) = peer_id {
                    if let Some(tx) = self.queue.dial.remove(&pid) {
                        let _ = tx.send(Err(error.into()));
                    }
                }
            }
            e => tracing::warn!("Unhandled swarm event: {:?}", e),
        };
        Ok(())
    }
    pub fn listen_on(&mut self, addr: Multiaddr) -> ListenerId {
        self.swarm.listen_on(addr).expect("")
    }
    pub fn pid(&self) -> &PeerId {
        self.swarm.local_peer_id()
    }
    pub async fn run(mut self) -> NetworkResult {
        Ok(loop {
            tokio::select! {
                Some(event) = self.swarm.next() => {
                    self.handle_event(event).await.expect("");
                }
                Some(cmd) = self.chan.cmd.recv() => {
                    self.handle_command(cmd).await.expect("Receiver not to be dropped");
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Signal received, shutting down...");
                    break;
                }
            }
        })
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

impl From<(Channels, Peer)> for Node {
    fn from(data: (Channels, Peer)) -> Self {
        Self::new(data.0, Swarm::from_peer(data.1))
    }
}

impl From<Channels> for Node {
    fn from(channels: Channels) -> Self {
        Self::from((channels, Peer::default()))
    }
}

impl From<Peer> for Node {
    fn from(peer: Peer) -> Self {
        Self::from((Channels::default(), peer))
    }
}
