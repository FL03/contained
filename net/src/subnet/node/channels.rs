/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements a dedicated channel manager for the node
*/
use crate::events::NetworkEvent;
use crate::subnet::layer::Command;
use tokio::sync::mpsc;

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
