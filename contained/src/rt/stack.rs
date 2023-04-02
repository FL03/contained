/*
    Appellation: rt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Environment, Workload};
use crate::prelude::{EnvId, Shared, WorkloadId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub enum StackFrame {
    Env(Environment),
    Workload(Workload),
}

/// The stack is a collection of all the running environments and workloads.
#[derive(Debug, Default)]
pub struct Stack {
    pub envs: RwLock<HashMap<EnvId, Shared<Environment>>>,
    pub workloads: RwLock<HashMap<WorkloadId, Shared<Workload>>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            envs: RwLock::new(HashMap::new()),
            workloads: RwLock::new(HashMap::new()),
        }
    }
    pub fn add_workload(&self, workload: Workload) {
        self.workloads
            .write()
            .unwrap()
            .insert(workload.id(), Arc::new(Mutex::new(workload)));
    }
    pub fn register(&self, env: Environment) {
        self.envs
            .write()
            .unwrap()
            .insert(env.clone().id, Arc::new(Mutex::new(env)));
    }
    pub fn remove_workload(&self, id: WorkloadId) {
        self.workloads.write().unwrap().remove(&id);
    }
    pub fn unregister(&self, id: EnvId) {
        self.envs.write().unwrap().remove(&id);
    }
}
