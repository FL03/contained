/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{layer::*, Space, Workload};
use crate::connect::Connection;
use crate::music::neo::triads::*;
use crate::prelude::{Error, SpaceId, WorkloadId};

use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::mpsc;
use wasmer::Module;

pub struct RuntimeState {
    pub spaces: RwLock<HashMap<SpaceId, Space>>,
    pub workloads: RwLock<HashMap<WorkloadId, Workload>>,
}

pub struct Runtime {
    con: Connection,
    command: mpsc::Receiver<Command>,
    event: mpsc::Sender<SystemEvent>,
    state: RuntimeState,
}

impl Runtime {
    pub async fn handle_request(&self, request: Command) -> Result<SystemEvent, Error> {
        match request {
            Command::AddTriad { id, .. } => {
                let triad = Triad::new(0.into(), Triads::Major);
                self.state
                    .spaces
                    .write()
                    .unwrap()
                    .insert(id.clone(), Space::new(triad));
                Ok(SystemEvent::TriadAdded { id })
            }
            Command::RemoveTriad { id } => {
                self.state.spaces.write().unwrap().remove(&id);
                Ok(SystemEvent::TriadRemoved { id })
            }
            Command::AddWorkload { id, module } => {
                // self.state.workloads.write().unwrap().insert(id.clone(), Workload::new(module, Module::new(vec![])));
                Ok(SystemEvent::WorkloadAdded { id })
            }
            Command::RemoveWorkload { id } => {
                self.state.workloads.write().unwrap().remove(&id);
                Ok(SystemEvent::WorkloadRemoved { id })
            }
            Command::RunWorkload {
                triad_id,
                workload_id,
            } => {
                let workload = self
                    .state
                    .workloads
                    .read()
                    .unwrap()
                    .get(&workload_id)
                    .unwrap();
                let triad = self.state.spaces.read().unwrap().get(&triad_id).unwrap();
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
                    let res = self.handle_request(req).await?;
                    self.event.send(res).await.expect("");
                }
            }
        }
    }
}
