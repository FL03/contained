/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{
    clients::frame::Frame,
    events::{Event, EventLoop, Events},
    proto::mainnet::Mainnet,
};
use either::Either;
use libp2p::kad::{self, KademliaEvent, QueryResult};
use libp2p::{
    mdns,
    multiaddr::Protocol,
    swarm::{ConnectionHandlerUpgrErr, SwarmEvent},
    Swarm,
};
use tokio::{
    io,
    sync::{mpsc, oneshot},
};
use tokio_stream::StreamExt;

pub struct Runtime {
    action: mpsc::Receiver<Frame>,
    event: mpsc::Sender<Event>,
    stack: EventLoop,
    swarm: Swarm<Mainnet>,
}

impl Runtime {
    pub fn new(
        action: mpsc::Receiver<Frame>,
        event: mpsc::Sender<Event>,
        swarm: Swarm<Mainnet>,
    ) -> Self {
        Self {
            action,
            event,
            stack: Default::default(),
            swarm,
        }
    }
    pub fn action(self) -> mpsc::Receiver<Frame> {
        self.action
    }
    pub fn event(self) -> mpsc::Sender<Event> {
        self.event
    }
    pub fn pending(self) -> EventLoop {
        self.stack
    }
    pub async fn handle_event(
        &mut self,
        event: SwarmEvent<Events, Either<ConnectionHandlerUpgrErr<io::Error>, io::Error>>,
    ) {
        match event {
            // Handle custom networking events
            SwarmEvent::Behaviour(b) => match b {
                Events::Kademlia(k) => match k {
                    KademliaEvent::OutboundQueryProgressed { id, result, .. } => match result {
                        QueryResult::GetProviders(Ok(get_providers)) => match get_providers {
                            kad::GetProvidersOk::FoundProviders { providers, .. } => {
                                if let Some(sender) = self.stack.get_providers.remove(&id) {
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
                                .stack
                                .start_providing
                                .remove(&id)
                                .expect("Completed query to be previously pending.");
                            let _ = sender.send(());
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Events::Mdns(m) => match m {
                    mdns::Event::Discovered(_disc) => {}
                    mdns::Event::Expired(_exp) => {}
                },
                Events::Ping(_) => {}
            },
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                match endpoint {
                    libp2p::core::ConnectedPoint::Dialer { .. } => if let Some(sender) = self.stack.dial.remove(&peer_id) {
                        let _ = sender.send(Ok(()));
                    },
                    _ => {},
                }
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {
                if let Some(pid) = peer_id {
                    if let Some(sender) = self.stack.dial.remove(&pid) {
                        let _ = sender.send(Err(Box::new(error)));
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
    pub async fn handle_command(&mut self, action: Frame) {
        match action {
            Frame::StartListening(act) => {
                let _ = match self.swarm.listen_on(act.address().clone()) {
                    Ok(_) => act.sender().send(Ok(())),
                    Err(e) => act.sender().send(Err(Box::new(e))),
                };
            }
            Frame::Dial(act) => {
                if let std::collections::hash_map::Entry::Vacant(e) =
                    self.stack.dial.entry(act.pid().clone())
                {
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(act.pid(), act.address().clone());
                    match self.swarm.dial(
                        act.address()
                            .clone()
                            .with(Protocol::P2p(act.pid().clone().into())),
                    ) {
                        Ok(()) => {
                            e.insert(act.sender());
                        }
                        Err(e) => {
                            let _ = act.sender().send(Err(Box::new(e)));
                        }
                    }
                } else {
                    todo!("Already dialing peer.");
                }
            }
            Frame::StartProviding(_act) => {}
            Frame::GetProviders(_act) => {}
        }
    }
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                event = self.swarm.next() => if let Some(_) = event {
                    // self.handle_event(e).await;
                }
            }
        }
    }
    pub fn swarm(self) -> Swarm<Mainnet> {
        self.swarm
    }
}
