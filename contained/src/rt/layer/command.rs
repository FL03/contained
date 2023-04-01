/*
    Appellation: command <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::prelude::{EnvId, Resultant, WorkloadId};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Command {
    Add {
        id: WorkloadId,
        module: u32,
        sender: Sender<Resultant>,
    },
    Register {
        id: EnvId,
        value: u32,
        sender: Sender<Resultant>,
    },
    Remove {
        id: WorkloadId,
        sender: Sender<Resultant>,
    },
    Run {
        env: EnvId,
        workload_id: WorkloadId,
        sender: Sender<Resultant>,
    },
    Unregister {
        id: EnvId,
        sender: Sender<Resultant>,
    },
}

impl Command {
    pub fn add_workload(id: WorkloadId, module: u32, sender: Sender<Resultant>) -> Self {
        Self::Add { id, module, sender }
    }
    pub fn register(id: EnvId, value: u32, sender: Sender<Resultant>) -> Self {
        Self::Register { id, value, sender }
    }
    pub fn remove_workload(id: WorkloadId, sender: Sender<Resultant>) -> Self {
        Self::Remove { id, sender }
    }
    pub fn run(env: EnvId, workload_id: WorkloadId, sender: Sender<Resultant>) -> Self {
        Self::Run {
            env,
            workload_id,
            sender,
        }
    }
    pub fn unregister(id: EnvId, sender: Sender<Resultant>) -> Self {
        Self::Unregister { id, sender }
    }
}
