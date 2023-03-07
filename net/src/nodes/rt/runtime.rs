/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This modules implements the network runtime;
*/
use crate::events::Event;
use crate::mainnet::{Mainnet, NetworkEvent};
use crate::nodes::rt::{eloop::EventLoop, ops::Frame};
use libp2p::multiaddr::Protocol;
use libp2p::swarm::{SwarmEvent, THandlerErr};
use libp2p::Swarm;
use std::collections::hash_map;
use tokio::{sync::mpsc, task::JoinHandle};
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
    pub async fn handle_event(&mut self, event: SwarmEvent<NetworkEvent, THandlerErr<Mainnet>>) {
        self.stack.handle_event(event, &mut self.swarm).await;
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
                if let hash_map::Entry::Vacant(e) = self.stack.dial.entry(*act.pid()) {
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(act.pid(), act.address().clone());
                    let dialopts = act
                        .address()
                        .clone()
                        .with(Protocol::P2p((*act.pid()).into()));
                    match self.swarm.dial(dialopts) {
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
                Some(act) = self.action.recv() => {
                    self.handle_command(act).await;
                },
                Some(event) = self.swarm.next() => {
                    self.handle_event(event).await;
                },
            }
        }
    }
    pub fn spawn(self) -> JoinHandle<()> {
        tokio::spawn(self.run())
    }
    pub fn swarm(self) -> Swarm<Mainnet> {
        self.swarm
    }
}
