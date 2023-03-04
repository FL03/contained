/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{clients::actions::Action, events::{Event, Events, EventLoop}, proto::Conduct};
use either::Either;
use futures::{Stream, StreamExt};
use libp2p::{swarm::{SwarmEvent, ConnectionHandlerUpgrErr}, Swarm};
use tokio::sync::mpsc;


pub struct Runtime {
    action: mpsc::Receiver<Action>,
    event: mpsc::Sender<Event>,
    stack: EventLoop,
    swarm: Swarm<Conduct>,
}

impl Runtime {
    pub fn new(
        action: mpsc::Receiver<Action>,
        event: mpsc::Sender<Event>,
        swarm: Swarm<Conduct>,
    ) -> Self {
        Self {
            action,
            event,
            stack: Default::default(),
            swarm,
        }
    }
    pub fn action(self) -> mpsc::Receiver<Action> {
        self.action
    }
    pub fn event(self) -> mpsc::Sender<Event> {
        self.event
    }
    pub async fn handle_event(&mut self, event: SwarmEvent<Events, Either<ConnectionHandlerUpgrErr<std::io::Error>, std::io::Error>> ) {

    }
    pub async fn handle_command(&mut self, action: Action) {
        
    }
    // pub async fn run(mut self) {
    //     loop {
    //         tokio::select! {
    //             event = self.swarm.next() => self.handle_event(event.expect("Swarm stream to be infinite.")).await  ,
    //             command = self.action.next() => match command {
    //                 Some(c) => self.handle_command(c).await,
    //                 // Command channel closed, thus shutting down the network event loop.
    //                 None=>  return,
    //             },
    //         }
    //     }
    // }
    pub fn swarm(self) -> Swarm<Conduct> {
        self.swarm
    }
}