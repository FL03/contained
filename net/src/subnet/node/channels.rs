/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements a dedicated channel manager for the node
*/
use crate::events::NetworkEvent;
use crate::subnet::{layer::Command, Client};
use tokio::sync::mpsc;

pub type CommandRx = mpsc::Receiver<Command>;
pub type CommandTx = mpsc::Sender<Command>;
pub type NetworkEventRx = mpsc::Receiver<NetworkEvent>;
pub type NetworkEventTx = mpsc::Sender<NetworkEvent>;

#[derive(Debug)]
pub struct Channels {
    pub cmd: CommandRx,
    pub event: NetworkEventTx,
}

impl Channels {
    pub fn new(cmd: CommandRx, event: NetworkEventTx) -> Self {
        Self { cmd, event }
    }
    pub fn with_capacity(capacity: usize) -> (Self, Client, NetworkEventRx) {
        let (cmd_tx, cmd_rx) = mpsc::channel(capacity);
        let (event_tx, event_rx) = mpsc::channel(capacity);
        (Self::new(cmd_rx, event_tx), Client::new(cmd_tx), event_rx)
    }
    pub fn command(&self) -> &CommandRx {
        &self.cmd
    }
    pub fn event(&self) -> &NetworkEventTx {
        &self.event
    }
}

impl Default for Channels {
    fn default() -> Self {
        Self::with_capacity(9).0
    }
}
