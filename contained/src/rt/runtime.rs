/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{reqres::*, Space, Workload};
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
    request: mpsc::Receiver<Requests>,
    response: mpsc::Sender<Responses>,
    state: RuntimeState,
}

impl Runtime {
    pub async fn handle_request(&self, request: Requests) -> Result<Responses, Error> {
        match request {
            Requests::AddTriad { id, .. } => {
                let triad = Triad::new(0.into(), Triads::Major);
                self.state
                    .spaces
                    .write()
                    .unwrap()
                    .insert(id.clone(), Space::new(triad));
                Ok(Responses::TriadAdded { id })
            }
            Requests::RemoveTriad { id } => {
                self.state.spaces.write().unwrap().remove(&id);
                Ok(Responses::TriadRemoved { id })
            }
            Requests::AddWorkload { id, module } => {
                // self.state.workloads.write().unwrap().insert(id.clone(), Workload::new(module, Module::new(vec![])));
                Ok(Responses::WorkloadAdded { id })
            }
            Requests::RemoveWorkload { id } => {
                self.state.workloads.write().unwrap().remove(&id);
                Ok(Responses::WorkloadRemoved { id })
            }
            Requests::RunWorkload {
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
                Ok(Responses::WorkloadRun {
                    triad_id,
                    workload_id,
                })
            }
            Requests::None => Ok(Responses::None),
        }
    }

    pub async fn run(mut self) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(req) = self.request.recv() => {
                    let res = self.handle_request(req).await?;
                    self.response.send(res).await.expect("");
                }
            }
        }
    }
}
