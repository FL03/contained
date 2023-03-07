/*
    Appellation: event_loop <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{
    mainnet::{Mainnet, NetworkEvent},
    NetResult,
};
use libp2p::kad::{self, KademliaEvent, QueryId, QueryResult};
use libp2p::multiaddr::Protocol;
use libp2p::swarm::{SwarmEvent, THandlerErr};
use libp2p::{mdns, PeerId, Swarm};
use std::collections::{HashMap, HashSet};
use tokio::sync::oneshot;

#[derive(Debug, Default)]
pub struct EventLoop {
    pub dial: HashMap<PeerId, oneshot::Sender<NetResult>>,
    pub start_providing: HashMap<QueryId, oneshot::Sender<()>>,
    pub get_providers: HashMap<QueryId, oneshot::Sender<HashSet<PeerId>>>,
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
    pub async fn handle_event(
        &mut self,
        event: SwarmEvent<NetworkEvent, THandlerErr<Mainnet>>,
        swarm: &mut Swarm<Mainnet>,
    ) {
        match event {
            // Handle custom networking events
            SwarmEvent::Behaviour(b) => match b {
                NetworkEvent::Kademlia(k) => match k {
                    KademliaEvent::OutboundQueryProgressed { id, result, .. } => match result {
                        QueryResult::GetProviders(Ok(get_providers)) => match get_providers {
                            kad::GetProvidersOk::FoundProviders { providers, .. } => {
                                if let Some(sender) = self.get_providers.remove(&id) {
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
                                .start_providing
                                .remove(&id)
                                .expect("Completed query to be previously pending.");
                            let _ = sender.send(());
                        }
                        _ => {}
                    },
                    _ => {}
                },
                NetworkEvent::Mdns(mdns_event) => match mdns_event {
                    mdns::Event::Discovered(_disc) => {}
                    mdns::Event::Expired(_exp) => {}
                },
                NetworkEvent::Ping(_) => {}
            },
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                if let libp2p::core::ConnectedPoint::Dialer { .. } = endpoint {
                    if let Some(sender) = self.dial.remove(&peer_id) {
                        let _ = sender.send(Ok(()));
                    }
                }
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {
                if let Some(pid) = peer_id {
                    if let Some(sender) = self.dial.remove(&pid) {
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
}
