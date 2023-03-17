/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This modules implements the network runtime;
*/
use super::{exec::Executor, frame::Frame};
use crate::events::NetworkEvent;

use tokio::sync::mpsc;

pub struct Runtime {
    pub(crate) action: mpsc::Receiver<Frame>,
    event: mpsc::Sender<NetworkEvent>,
    pub(crate) exec: Executor,
}

impl Runtime {
    pub fn new(
        action: mpsc::Receiver<Frame>,
        event: mpsc::Sender<NetworkEvent>,
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
    pub fn event(self) -> mpsc::Sender<NetworkEvent> {
        self.event
    }
    pub fn pending(self) -> Executor {
        self.exec
    }
}
