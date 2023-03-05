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
use libp2p::{
    swarm::{ConnectionHandlerUpgrErr, SwarmEvent},
    Swarm,
};
use tokio::{io, sync::mpsc};
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
            SwarmEvent::Behaviour(_) => {},
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, num_established, concurrent_dial_errors, established_in } => {},
            SwarmEvent::ConnectionClosed { peer_id, endpoint, num_established, cause } => {},
            SwarmEvent::IncomingConnection { local_addr, send_back_addr } => {},
            SwarmEvent::IncomingConnectionError { local_addr, send_back_addr, error } => {},
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {},
            SwarmEvent::BannedPeer { peer_id, endpoint } => {},
            SwarmEvent::NewListenAddr { listener_id, address } => {},
            SwarmEvent::ExpiredListenAddr { listener_id, address } => {},
            SwarmEvent::ListenerClosed { listener_id, addresses, reason } => {},
            SwarmEvent::ListenerError { listener_id, error } => {},
            SwarmEvent::Dialing(_) => {},
        }
    }
    pub async fn handle_command(&mut self, action: Frame) {
        match action {
            Frame::StartListening(_act) => {}
            Frame::Dial(_act) => {}
            Frame::StartProviding(_act) => {}
            Frame::GetProviders(_act) => {}
        }
    }
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                event = self.swarm.next() => {

                }
            }
        }
    }
    pub fn swarm(self) -> Swarm<Mainnet> {
        self.swarm
    }
}
