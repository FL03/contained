/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{layer::*, Space, Workload};
use crate::music::neo::triads::*;
use crate::prelude::{Error, SpaceId, WorkloadId};

use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::mpsc;
use wasmer::Module;

pub struct Stack {
    pub spaces: RwLock<HashMap<SpaceId, Space>>,
    pub workloads: RwLock<HashMap<WorkloadId, Workload>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            spaces: RwLock::new(HashMap::new()),
            workloads: RwLock::new(HashMap::new()),
        }
    }
}

pub struct Runtime {
    command: mpsc::Receiver<Command>,
    event: mpsc::Sender<SystemEvent>,
    stack: Stack,
}

impl Runtime {
    pub fn new(command: mpsc::Receiver<Command>, event: mpsc::Sender<SystemEvent>) -> Self {
        Self {
            command,
            event,
            stack: Stack::new(),
        }
    }
    pub async fn handle_command(&self, request: Command) -> Result<SystemEvent, Error> {
        match request {
            Command::AddTriad { id, .. } => {
                let triad = Triad::new(0.into(), TriadClass::Major);
                self.stack
                    .spaces
                    .write()
                    .unwrap()
                    .insert(id.clone(), Space::new(triad));
                Ok(SystemEvent::TriadAdded { id })
            }
            Command::RemoveTriad { id } => {
                self.stack.spaces.write().unwrap().remove(&id);
                Ok(SystemEvent::TriadRemoved { id })
            }
            Command::AddWorkload { id, module } => {
                // self.state.workloads.write().unwrap().insert(id.clone(), Workload::new(module, Module::new(vec![])));
                Ok(SystemEvent::WorkloadAdded { id })
            }
            Command::RemoveWorkload { id } => {
                self.stack.workloads.write().unwrap().remove(&id);
                Ok(SystemEvent::WorkloadRemoved { id })
            }
            Command::RunWorkload {
                triad_id,
                workload_id,
            } => {
                let workload = self
                    .stack
                    .workloads
                    .read()
                    .unwrap()
                    .get(&workload_id)
                    .unwrap();
                let triad = self.stack.spaces.read().unwrap().get(&triad_id).unwrap();
                Ok(SystemEvent::WorkloadRun {
                    triad_id,
                    workload_id,
                })
            }
            Command::None => Ok(SystemEvent::None),
        }
    }
    pub async fn run(mut self) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(req) = self.command.recv() => {
                    let res = self.handle_command(req).await?;
                    self.event.send(res).await.expect("");
                }
            }
        }
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<Result<(), Error>> {
        tokio::spawn(self.run())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(mpsc::channel(100).1, mpsc::channel(100).0)
    }
}
