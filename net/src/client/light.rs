/*
    Appellation: light <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A light client is one that does not participate in the consensus process, but rather only stores the blockchain and can query it.
*/
use crate::subnet::layer::Command;
use tokio::sync::mpsc;

pub struct LightClient {
    pub cmd: mpsc::Sender<Command>,
}

impl LightClient {
    pub fn new(capacity: usize) -> Self {
        Self {
            cmd: mpsc::channel(capacity).0,
        }
    }
}

impl Default for LightClient {
    fn default() -> Self {
        Self::new(9)
    }
}
