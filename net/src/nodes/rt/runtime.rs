/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This modules implements the network runtime;
*/
use crate::events::ClientEvent;
use crate::mainnet::{Mainnet, NetworkEvent};
use crate::nodes::rt::{exec::Executor, frame::Frame};
use libp2p::multiaddr::Protocol;
use libp2p::swarm::{SwarmEvent, THandlerErr};
use libp2p::Swarm;
use std::collections::hash_map;
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_stream::StreamExt;

pub struct Runtime {
    pub action: mpsc::Receiver<Frame>,
    event: mpsc::Sender<ClientEvent>,
    pub exec: Executor,
}

impl Runtime {
    pub fn new(
        action: mpsc::Receiver<Frame>,
        event: mpsc::Sender<ClientEvent>,
        exec: Executor,
    ) -> Self {
        Self {
            action,
            event,
            exec,
        }
    }
    pub fn action(self) -> mpsc::Receiver<Frame> {
        self.action
    }
    pub fn event(self) -> mpsc::Sender<ClientEvent> {
        self.event
    }
    pub fn pending(self) -> Executor {
        self.exec
    }
    pub async fn run(mut self, swarm: &mut Swarm<Mainnet>) {
        loop {
            tokio::select! {
                Some(act) = self.action.recv() => {
                    self.exec.handle_command(act, swarm).await;
                },
                Some(event) = swarm.next() => {
                    self.exec.handle_event(event, swarm).await;
                },
            }
        }
    }
    // pub fn spawn(self) -> JoinHandle<()> {
    //     tokio::spawn(self.run())
    // }
}
