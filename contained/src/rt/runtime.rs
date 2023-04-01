/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{layer::*, Stack};
use crate::prelude::Error;

use crate::music::neo::tonnetz::Cluster;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

pub struct Runtime {
    command: CommandReceiver,
    event: SysEventSender,
    stack: Stack,
}

impl Runtime {
    pub fn new(command: CommandReceiver, event: SysEventSender) -> Self {
        Self {
            command,
            event,
            stack: Stack::new(),
        }
    }
    pub async fn handle_command(&self, request: Command) -> Result<ClusterEvent, Error> {
        match request {
            Command::Register { id, sender, value } => {
                self.stack.envs.write().unwrap().insert(id.clone(), sender);
                Ok(ClusterEvent::TriadAdded { id })
            }
            _ => Ok(ClusterEvent::None),
        }
    }
    pub async fn run(mut self) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(req) = self.command.recv() => {
                    let res = self.handle_command(req).await?;
                    self.event.send(res).await.expect("");
                },
                else => {
                }
            }
        }
    }
    pub fn spawn(self) -> JoinHandle<Result<(), Error>> {
        tokio::spawn(self.run())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(mpsc::channel(100).1, mpsc::channel(100).0)
    }
}
