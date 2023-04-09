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
use crate::peers::Peer;
use crate::NetworkResult;
use futures::StreamExt;
use libp2p::kad::{self, KademliaEvent, QueryResult};
use libp2p::swarm::{SwarmEvent, THandlerErr};
use libp2p::{mdns, request_response};
use libp2p::{multiaddr::Protocol, Multiaddr, PeerId, Swarm};
use tokio::sync::oneshot;

pub struct SubnetConfig {
    pub addr: Multiaddr,
}

impl SubnetConfig {
    pub fn new() -> Self {
        Self {
            addr: "ip4/0.0.0.0/tcp/9090".parse().unwrap(),
        }
    }
}



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
    pub fn dial(&mut self, addr: Multiaddr, pid: PeerId) -> NetworkResult<()> {
        let opts = addr.with(Protocol::P2p((pid).into()));
        self.swarm.dial(opts)?;
        Ok(())
    }
    pub async fn handle_command(&mut self, action: Command) -> NetworkResult {
        match action {
            Command::Listen { addr, tx } => {
                let msg = self.swarm.listen_on(addr).map_err(|e| e.into());
                tx.send(msg).expect("Receiver to be still open.");
            }
            Command::Dial {
                addr,
                pid,
                tx,
            } => match self.queue.dial.entry(pid) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    tracing::warn!("The peer ({}) is already being dialed", pid);
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(&pid, addr.clone());
                    let dialopts = addr.with(Protocol::P2p((pid).into()));
                    match self.swarm.dial(dialopts) {
                        Ok(_) => {
                            e.insert(tx);
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e.into()));
                        }
                    }
                }
            },
            Command::Provide { .. } => {}
            Command::Providers { .. } => {}
            Command::Request { payload, peer, tx } => {
                let request_id = self.swarm
                    .behaviour_mut()
                    .reqres
                    .send_request(&peer, reqres::Request::new(payload));
                self.queue.requests.insert(request_id, tx);
            }
            Command::Respond { payload, channel } => {
                self.swarm
                    .behaviour_mut()
                    .reqres
                    .send_response(channel, reqres::Response::new().with_data(payload))
                    .expect("Connection to peer to be still open.");
            }
        };
        Ok(())
    }
    /// Handle events from the swarm; the stateful network manager
    pub async fn handle_event(&mut self, event: SwarmEvent<SubnetEvent, THandlerErr<Subnet>>) {
        match event {
            // Handle custom networking events
            SwarmEvent::Behaviour(subnet) => match subnet {
                SubnetEvent::Kademlia(kademlia) => match kademlia {
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
                    mdns::Event::Discovered(disc) => {
                        for (pid, addr) in disc {
                            tracing::info!("Discovered peer: {} at {}", pid, addr);
                        }
                    }
                    mdns::Event::Expired(_exp) => {}
                },
                SubnetEvent::RequestResponse(evnt) => match evnt {
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
                tracing::info!(
                    "Local node is listening on {:?}",
                    address.with(Protocol::P2p(local_peer_id.into()))
                );
            }
            SwarmEvent::Dialing(pid) => {
                tracing::info!("Dialing peer: {}", pid);
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
    pub fn listen_on(&mut self, addr: Multiaddr) -> libp2p::core::transport::ListenerId {
        self.swarm.listen_on(addr).expect("")
    }
    pub fn pid(&self) -> &PeerId {
        self.swarm.local_peer_id()
    }
    pub async fn run(mut self) -> NetworkResult {
        Ok(loop {
            tokio::select! {
                Some(event) = self.swarm.next() => {
                    self.handle_event(event).await;
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
        let swarm = data.1.swarm();

        Self::new(data.0, swarm)
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
