/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::reqres::*;
use super::Workload;
use crate::connect::Connection;
use crate::core::Error;
use crate::music::neo::triads::Triad;

use std::collections::HashMap;
use std::sync::RwLock;

pub struct RuntimeState {
    pub triads: RwLock<HashMap<u32, Triad>>,
    pub workloads: RwLock<HashMap<String, Workload>>,
}

pub struct Runtime {
    con: Connection,
    state: RuntimeState,
}

impl Runtime {
    pub async fn handle_request(&self, request: Request) -> Result<Response, Error> {
        match request {
            Request::AddTriad(id, value) => Ok(Response::TriadAdded(id)),
            Request::RemoveTriad(id) => {
                self.state.triads.write().unwrap().remove(&id);
                Ok(Response::TriadRemoved(id))
            }
            Request::AddWorkload(id, module) => Ok(Response::WorkloadAdded(id)),
            Request::RemoveWorkload(id) => {
                self.state.workloads.write().unwrap().remove(&id);
                Ok(Response::WorkloadRemoved(id))
            }
            Request::RunWorkload(workload_id, triad_id) => {
                let workload = self
                    .state
                    .workloads
                    .read()
                    .unwrap()
                    .get(&workload_id)
                    .unwrap();
                let triad = self.state.triads.read().unwrap().get(&triad_id).unwrap();
                Ok(Response::WorkloadRun(workload_id, triad_id))
            }
            Request::None => Ok(Response::None),
        }
    }
}
