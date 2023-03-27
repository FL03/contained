/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::reqres::*;
use super::{Space, Workload};
use crate::connect::Connection;
use crate::core::Error;

use std::collections::HashMap;
use std::sync::RwLock;

pub struct RuntimeState {
    pub spaces: RwLock<HashMap<u32, Space>>,
    pub workloads: RwLock<HashMap<String, Workload>>,
}

pub struct Runtime {
    con: Connection,
    state: RuntimeState,
}

impl Runtime {
    pub async fn handle_request(&self, request: Request) -> Result<Response, Error> {
        match request {
            Request::AddTriad { id, .. } => Ok(Response::TriadAdded { id }),
            Request::RemoveTriad { id } => {
                self.state.spaces.write().unwrap().remove(&id);
                Ok(Response::TriadRemoved { id })
            }
            Request::AddWorkload { id, .. } => Ok(Response::WorkloadAdded { id }),
            Request::RemoveWorkload { id } => {
                self.state.workloads.write().unwrap().remove(&id);
                Ok(Response::WorkloadRemoved { id })
            }
            Request::RunWorkload { triad_id, workload_id } => {
                let workload = self
                    .state
                    .workloads
                    .read()
                    .unwrap()
                    .get(&workload_id)
                    .unwrap();
                let triad = self.state.spaces.read().unwrap().get(&triad_id).unwrap();
                Ok(Response::WorkloadRun{ triad_id, workload_id })
            }
            Request::None => Ok(Response::None),
        }
    }
}
