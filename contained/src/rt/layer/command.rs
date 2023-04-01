/*
    Appellation: command <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::prelude::{EnvId, Resultant, WorkloadId};
use crate::rt::{Environment, Workload};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Command {
    Add { workload: Workload },
    Register { env: Environment },
    Remove { id: WorkloadId },
    Run { env: EnvId, workload_id: WorkloadId },
    Unregister { id: EnvId },
}

impl Command {
    pub fn add_workload(workload: Workload) -> Self {
        Self::Add { workload }
    }
    pub fn register(env: Environment) -> Self {
        Self::Register { env }
    }
    pub fn remove_workload(id: WorkloadId) -> Self {
        Self::Remove { id }
    }
    pub fn run(env: EnvId, workload_id: WorkloadId) -> Self {
        Self::Run { env, workload_id }
    }
    pub fn unregister(id: EnvId) -> Self {
        Self::Unregister { id }
    }
}
