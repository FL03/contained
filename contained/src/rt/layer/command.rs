/*
    Appellation: command <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::prelude::{EnvId, Resultant, WorkloadId};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Command {
    AddTriad {
        id: EnvId,
        value: u32,
        sender: Sender<Resultant>,
    },
    RemoveTriad {
        id: EnvId,
        sender: Sender<Resultant>,
    },
    AddWorkload {
        id: WorkloadId,
        module: u32,
        sender: Sender<Resultant>,
    },
    RemoveWorkload {
        id: WorkloadId,
        sender: Sender<Resultant>,
    },
    RunWorkload {
        env: EnvId,
        workload_id: WorkloadId,
        sender: Sender<Resultant>,
    },
}

impl Command {
    pub fn add_triad(id: EnvId, value: u32, sender: Sender<Resultant>) -> Self {
        Self::AddTriad { id, value, sender }
    }
    pub fn remove_triad(id: EnvId, sender: Sender<Resultant>) -> Self {
        Self::RemoveTriad { id, sender }
    }
    pub fn add_workload(id: WorkloadId, module: u32, sender: Sender<Resultant>) -> Self {
        Self::AddWorkload { id, module, sender }
    }
    pub fn remove_workload(id: WorkloadId, sender: Sender<Resultant>) -> Self {
        Self::RemoveWorkload { id, sender }
    }
    pub fn run_workload(env: EnvId, workload_id: WorkloadId, sender: Sender<Resultant>) -> Self {
        Self::RunWorkload {
            env,
            workload_id,
            sender,
        }
    }
}
